use crate::application::ports::input::RecordUseCase;
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
    // Using simple Form extract would be better for HTML forms, but using Json as requested in plan or default
    // Wait, HTML form usually sends x-www-form-urlencoded.
    // Spec doesn't strictly say, but standard web app uses Form.
    // I will use Form to support the UI I'm building.
    // Need axum::Form
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
