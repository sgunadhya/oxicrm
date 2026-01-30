use crate::application::ports::input::RecordUseCase;
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
}

#[derive(Deserialize)]
pub struct MoveCardPayload {
    pub new_stage: OpportunityStage,
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
