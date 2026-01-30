use crate::application::ports::input::{CreateWorkspaceUseCase, RecordUseCase};
use crate::application::use_cases::create_workspace::CreateWorkspace;
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
