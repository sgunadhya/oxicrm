use crate::application::ports::input::{CreateWorkspaceUseCase, RecordUseCase};
use crate::application::ports::output::{CompanyRepository, PersonRepository};
use crate::application::use_cases::create_company::CreateCompany;
use crate::application::use_cases::create_person::CreatePerson;
use crate::application::use_cases::create_workspace::CreateWorkspace;
use crate::application::use_cases::manage_company::ManageCompany;
use crate::application::use_cases::manage_person::ManagePerson;
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
