use crate::application::use_cases::manage_custom_object_data::ManageCustomObjectData;
use crate::application::use_cases::manage_metadata::ManageMetadata;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct CustomObjectAppState {
    pub manage_custom_object_data: Arc<ManageCustomObjectData>,
    pub manage_metadata: Arc<ManageMetadata>,
}

#[derive(Deserialize)]
pub struct CreateRecordPayload {
    pub properties: serde_json::Value,
}

#[derive(Deserialize)]
pub struct UpdateRecordPayload {
    pub properties: serde_json::Value,
}

pub async fn list_records_handler(
    State(state): State<CustomObjectAppState>,
    Path(object_id): Path<Uuid>,
) -> impl IntoResponse {
    match state
        .manage_custom_object_data
        .list_records(object_id)
        .await
    {
        Ok(records) => Json(records).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

pub async fn create_record_handler(
    State(state): State<CustomObjectAppState>,
    Path(object_id): Path<Uuid>,
    Json(payload): Json<CreateRecordPayload>,
) -> impl IntoResponse {
    match state
        .manage_custom_object_data
        .create_record(object_id, payload.properties, Uuid::default())
        .await
    {
        Ok(record) => (StatusCode::CREATED, Json(record)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

pub async fn get_record_handler(
    State(state): State<CustomObjectAppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.manage_custom_object_data.get_record(id).await {
        Ok(Some(record)) => Json(record).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "Record not found" })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

pub async fn update_record_handler(
    State(state): State<CustomObjectAppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateRecordPayload>,
) -> impl IntoResponse {
    match state
        .manage_custom_object_data
        .update_record(id, payload.properties)
        .await
    {
        Ok(record) => Json(record).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

pub async fn delete_record_handler(
    State(state): State<CustomObjectAppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.manage_custom_object_data.delete_record(id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}
