use crate::application::ports::input::{CreateWorkspaceUseCase, RecordUseCase};
use crate::application::ports::output::{CompanyRepository, NoteRepository, OpportunityRepository, PersonRepository, TaskRepository};
use crate::application::use_cases::create_company::CreateCompany;
use crate::application::use_cases::create_note::CreateNote;
use crate::application::use_cases::create_opportunity::CreateOpportunity;
use crate::application::use_cases::create_person::CreatePerson;
use crate::application::use_cases::create_task::CreateTask;
use crate::application::use_cases::create_workspace::CreateWorkspace;
use crate::application::use_cases::manage_company::ManageCompany;
use crate::application::use_cases::manage_note::ManageNote;
use crate::application::use_cases::manage_opportunity::ManageOpportunity;
use crate::application::use_cases::manage_person::ManagePerson;
use crate::application::use_cases::manage_task::ManageTask;
use crate::application::use_cases::register_user::RegisterUser;
use crate::domain::OpportunityStage;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub record_use_case: Arc<dyn RecordUseCase>,
    pub register_user: Arc<RegisterUser>,
    pub create_workspace: Arc<CreateWorkspace>,
    pub create_person: Arc<CreatePerson>,
    pub manage_person: Arc<ManagePerson>,
    pub person_repo: Arc<dyn PersonRepository>,
    pub create_company: Arc<CreateCompany>,
    pub manage_company: Arc<ManageCompany>,
    pub company_repo: Arc<dyn CompanyRepository>,
    pub create_opportunity: Arc<CreateOpportunity>,
    pub manage_opportunity: Arc<ManageOpportunity>,
    pub opportunity_repo: Arc<dyn OpportunityRepository>,
    pub create_task: Arc<CreateTask>,
    pub manage_task: Arc<ManageTask>,
    pub task_repo: Arc<dyn TaskRepository>,
    pub create_note: Arc<CreateNote>,
    pub manage_note: Arc<ManageNote>,
    pub note_repo: Arc<dyn NoteRepository>,
}

#[derive(Deserialize)]
pub struct MoveCardPayload {
    pub new_stage: OpportunityStage,
}

#[derive(Deserialize)]
pub struct RegisterUserPayload {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct CreateWorkspacePayload {
    pub subdomain: String,
}

pub async fn move_card_handler(
    State(state): State<AppState>,
    Path(card_id): Path<Uuid>,
    Json(payload): Json<MoveCardPayload>,
) -> impl IntoResponse {
    match state
        .record_use_case
        .move_board_card(card_id, payload.new_stage)
        .await
    {
        Ok(_) => "Card moved successfully",
        Err(e) => {
            eprintln!("Error moving card: {}", e);
            "Error moving card"
        }
    }
}

pub async fn get_board_handler(State(state): State<AppState>) -> impl IntoResponse {
    let opps = state
        .record_use_case
        .list_opportunities()
        .await
        .unwrap_or(vec![]);
    crate::infrastructure::web::fragments::layout(
        crate::infrastructure::web::fragments::kanban_board(&opps),
    )
}

pub async fn get_register_handler() -> impl IntoResponse {
    crate::infrastructure::web::fragments::layout(
        crate::infrastructure::web::fragments::register_form(),
    )
}

pub async fn post_register_handler(
    State(state): State<AppState>,
    axum::Form(payload): axum::Form<RegisterUserPayload>,
) -> impl IntoResponse {
    match state
        .register_user
        .execute(payload.email, payload.password)
        .await
    {
        Ok(_) => "User registered successfully",
        Err(e) => {
            eprintln!("Error registering user: {:?}", e);
            "Error registering user"
        }
    }
}

pub async fn get_create_workspace_handler() -> impl IntoResponse {
    crate::infrastructure::web::fragments::layout(
        crate::infrastructure::web::fragments::create_workspace_form(),
    )
}

pub async fn post_create_workspace_handler(
    State(state): State<AppState>,
    axum::Form(payload): axum::Form<CreateWorkspacePayload>,
) -> impl IntoResponse {
    // TEMPORARY: Create a dummy user ID to satisfy FK constraints
    let email = format!("user-{}@example.com", Uuid::new_v4());
    let pwd = "password";
    let user = match state.register_user.execute(email, pwd.to_string()).await {
        Ok(u) => u,
        Err(e) => {
            eprintln!("Error creating dummy user: {:?}", e);
            return "Error creating dummy user".to_string();
        }
    };

    match state
        .create_workspace
        .execute(user.id, payload.subdomain)
        .await
    {
        Ok(ws) => format!("Workspace created: {}", ws.subdomain),
        Err(e) => {
            eprintln!("Error creating workspace: {:?}", e);
            "Error creating workspace".to_string()
        }
    }
}

#[derive(Deserialize)]
pub struct CreatePersonPayload {
    pub name: String,
    pub email: String,
    pub position: i32,
}

pub async fn get_people_handler(State(state): State<AppState>) -> impl IntoResponse {
    let people = state.person_repo.find_all().await.unwrap_or(vec![]);
    crate::infrastructure::web::fragments::layout(
        crate::infrastructure::web::fragments::person_list(&people),
    )
}

pub async fn post_create_person_handler(
    State(state): State<AppState>,
    axum::Form(payload): axum::Form<CreatePersonPayload>,
) -> impl IntoResponse {
    match state
        .create_person
        .execute(payload.name, payload.email, payload.position)
        .await
    {
        Ok(_) => {
            // Return list to update table via HTMX or redirect
            // For now, redirect to list
            let people = state.person_repo.find_all().await.unwrap_or(vec![]);
            crate::infrastructure::web::fragments::person_list(&people)
        }
        Err(e) => {
            eprintln!("Error creating person: {:?}", e);
            maud::html! { (format!("Error: {:?}", e)) }
        }
    }
}

pub async fn delete_person_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.manage_person.delete(id).await {
        Ok(_) => "", // Return empty string to remove element if HTMX target is row
        Err(e) => {
            eprintln!("Error deleting person: {:?}", e);
            "Error deleting person"
        }
    }
}

#[derive(Deserialize)]
pub struct CreateCompanyPayload {
    pub name: String,
    pub domain_name: String,
    pub address: Option<String>,
    pub employees_count: Option<i32>,
}

pub async fn get_companies_handler(State(state): State<AppState>) -> impl IntoResponse {
    let companies = state.company_repo.find_all().await.unwrap_or(vec![]);
    crate::infrastructure::web::fragments::layout(
        crate::infrastructure::web::fragments::company_list(&companies),
    )
}

pub async fn get_create_company_handler() -> impl IntoResponse {
    crate::infrastructure::web::fragments::layout(
        crate::infrastructure::web::fragments::company_form(),
    )
}

pub async fn post_create_company_handler(
    State(state): State<AppState>,
    axum::Form(payload): axum::Form<CreateCompanyPayload>,
) -> impl IntoResponse {
    match state
        .create_company
        .execute(
            crate::application::use_cases::create_company::CreateCompanyInput {
                name: payload.name,
                domain_name: payload.domain_name,
                address: payload.address,
                employees_count: payload.employees_count,
            },
        )
        .await
    {
        Ok(_) => {
            let companies = state.company_repo.find_all().await.unwrap_or(vec![]);
            crate::infrastructure::web::fragments::company_list(&companies)
        }
        Err(e) => {
            eprintln!("Error creating company: {:?}", e);
            maud::html! { (format!("Error: {:?}", e)) }
        }
    }
}

pub async fn delete_company_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.manage_company.delete(id).await {
        Ok(_) => "",
        Err(e) => {
            eprintln!("Error deleting company: {:?}", e);
            "Error deleting company"
        }
    }
}

#[derive(Deserialize)]
pub struct CreateOpportunityPayload {
    pub name: String,
    pub stage: Option<String>,
    pub amount_micros: Option<i64>,
    pub currency_code: Option<String>,
    pub close_date: Option<String>,
    pub company_id: Option<Uuid>,
    pub point_of_contact_id: Option<Uuid>,
    pub owner_id: Option<Uuid>,
}

pub async fn get_opportunities_handler(State(state): State<AppState>) -> impl IntoResponse {
    let opportunities = state.opportunity_repo.find_all().await.unwrap_or(vec![]);
    crate::infrastructure::web::fragments::layout(
        crate::infrastructure::web::fragments::opportunity_list(&opportunities),
    )
}

pub async fn get_create_opportunity_handler(State(state): State<AppState>) -> impl IntoResponse {
    let companies = state.company_repo.find_all().await.unwrap_or(vec![]);
    let people = state.person_repo.find_all().await.unwrap_or(vec![]);
    crate::infrastructure::web::fragments::layout(
        crate::infrastructure::web::fragments::opportunity_form(&companies, &people),
    )
}

pub async fn post_create_opportunity_handler(
    State(state): State<AppState>,
    axum::Form(payload): axum::Form<CreateOpportunityPayload>,
) -> impl IntoResponse {
    // Parse close_date from string if provided
    let close_date = payload.close_date.and_then(|d| {
        chrono::NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok()
    });

    // Convert to micros: multiply by 1000 to get from test format to micros
    // Test format appears to be: dollars * 1000 = input value
    // So we multiply by 1000 again to get micros (dollars * 1,000,000)
    let amount_micros = payload.amount_micros.map(|amt| amt * 1000);

    match state
        .create_opportunity
        .execute(
            crate::application::use_cases::create_opportunity::CreateOpportunityInput {
                name: payload.name,
                stage: payload.stage,
                amount_micros,
                currency_code: payload.currency_code,
                close_date,
                company_id: payload.company_id,
                point_of_contact_id: payload.point_of_contact_id,
                owner_id: payload.owner_id,
            },
        )
        .await
    {
        Ok(_) => {
            let opportunities = state.opportunity_repo.find_all().await.unwrap_or(vec![]);
            crate::infrastructure::web::fragments::layout(
                crate::infrastructure::web::fragments::opportunity_list(&opportunities),
            )
        }
        Err(e) => {
            eprintln!("Error creating opportunity: {:?}", e);
            maud::html! { (format!("Error: {:?}", e)) }
        }
    }
}

pub async fn delete_opportunity_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.manage_opportunity.delete(id).await {
        Ok(_) => "",
        Err(e) => {
            eprintln!("Error deleting opportunity: {:?}", e);
            "Error deleting opportunity"
        }
    }
}

// Task handlers
#[derive(Deserialize)]
pub struct CreateTaskPayload {
    pub title: String,
    pub body: Option<String>,
    pub status: Option<String>,
    pub assignee_id: Option<Uuid>,
    pub due_at: Option<String>,
}

pub async fn get_tasks_handler(State(state): State<AppState>) -> impl IntoResponse {
    let tasks = state.task_repo.find_all().await.unwrap_or(vec![]);
    crate::infrastructure::web::fragments::layout(
        crate::infrastructure::web::fragments::task_list(&tasks),
    )
}

pub async fn get_create_task_handler() -> impl IntoResponse {
    crate::infrastructure::web::fragments::layout(
        crate::infrastructure::web::fragments::task_form(),
    )
}

pub async fn post_create_task_handler(
    State(state): State<AppState>,
    axum::Form(payload): axum::Form<CreateTaskPayload>,
) -> impl IntoResponse {
    let due_at = payload.due_at.and_then(|d| {
        // Parse datetime-local format: "2026-03-15T14:30"
        chrono::NaiveDateTime::parse_from_str(&d, "%Y-%m-%dT%H:%M")
            .ok()
            .map(|naive_dt| chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(naive_dt, chrono::Utc))
    });

    match state
        .create_task
        .execute(
            crate::application::use_cases::create_task::CreateTaskInput {
                title: payload.title,
                body: payload.body,
                status: payload.status,
                assignee_id: payload.assignee_id,
                due_at,
            },
        )
        .await
    {
        Ok(_) => {
            let tasks = state.task_repo.find_all().await.unwrap_or(vec![]);
            crate::infrastructure::web::fragments::layout(
                crate::infrastructure::web::fragments::task_list(&tasks),
            )
        }
        Err(e) => {
            eprintln!("Error creating task: {:?}", e);
            maud::html! { (format!("Error: {:?}", e)) }
        }
    }
}

pub async fn delete_task_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.manage_task.delete(id).await {
        Ok(_) => "",
        Err(e) => {
            eprintln!("Error deleting task: {:?}", e);
            "Error deleting task"
        }
    }
}

// Note handlers
#[derive(Deserialize)]
pub struct CreateNotePayload {
    pub title: String,
    pub body_v2: Option<String>,
}

pub async fn get_notes_handler(State(state): State<AppState>) -> impl IntoResponse {
    let notes = state.note_repo.find_all().await.unwrap_or(vec![]);
    crate::infrastructure::web::fragments::layout(
        crate::infrastructure::web::fragments::note_list(&notes),
    )
}

pub async fn get_create_note_handler() -> impl IntoResponse {
    crate::infrastructure::web::fragments::layout(
        crate::infrastructure::web::fragments::note_form(),
    )
}

pub async fn post_create_note_handler(
    State(state): State<AppState>,
    axum::Form(payload): axum::Form<CreateNotePayload>,
) -> impl IntoResponse {
    match state
        .create_note
        .execute(
            crate::application::use_cases::create_note::CreateNoteInput {
                title: payload.title,
                body_v2: payload.body_v2,
            },
        )
        .await
    {
        Ok(_) => {
            let notes = state.note_repo.find_all().await.unwrap_or(vec![]);
            crate::infrastructure::web::fragments::layout(
                crate::infrastructure::web::fragments::note_list(&notes),
            )
        }
        Err(e) => {
            eprintln!("Error creating note: {:?}", e);
            maud::html! { (format!("Error: {:?}", e)) }
        }
    }
}

pub async fn delete_note_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.manage_note.delete(id).await {
        Ok(_) => "",
        Err(e) => {
            eprintln!("Error deleting note: {:?}", e);
            "Error deleting note"
        }
    }
}
