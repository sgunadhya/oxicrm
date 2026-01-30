use crate::application::use_cases::manage_metadata::ManageMetadata;
use crate::application::use_cases::manage_view::ManageView;
use crate::domain::metadata::{FieldType, ObjectMetadata, ViewType};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct MetadataAppState {
    pub manage_metadata: Arc<ManageMetadata>,
    pub manage_view: Arc<ManageView>,
}

#[derive(Deserialize)]
pub struct CreateObjectPayload {
    pub name_singular: String,
    pub name_plural: String,
    pub description: Option<String>,
}

// Custom Field Payload
#[derive(Deserialize)]
pub struct CreateFieldPayload {
    pub name: String,
    pub field_type: String, // "Text", "Number", etc.
    pub settings: Option<serde_json::Value>,
}

// View Payload
#[derive(Deserialize)]
pub struct CreateViewPayload {
    pub object_metadata_id: Uuid,
    pub name: String,
    pub view_type: String, // "Table", "Kanban", etc.
    pub filters: serde_json::Value,
    pub sort: serde_json::Value,
}

#[derive(Deserialize)]
pub struct UpdateViewPayload {
    pub name: Option<String>,
    pub filters: Option<serde_json::Value>,
    pub sort: Option<serde_json::Value>,
}

#[derive(Deserialize)]
pub struct ListViewQuery {
    pub object_metadata_id: Uuid,
}

// --- Handlers ---

pub async fn get_schema_handler(State(state): State<MetadataAppState>) -> impl IntoResponse {
    match state.manage_metadata.get_schema().await {
        Ok(schema) => Json(schema).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

pub async fn create_object_handler(
    State(state): State<MetadataAppState>,
    Json(payload): Json<CreateObjectPayload>,
) -> impl IntoResponse {
    match state
        .manage_metadata
        .create_object(
            payload.name_singular,
            payload.name_plural,
            payload.description,
            Uuid::default(), // TODO: Get from auth
        )
        .await
    {
        Ok(object) => (StatusCode::CREATED, Json(object)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

pub async fn create_field_handler(
    State(state): State<MetadataAppState>,
    Path(object_id): Path<Uuid>,
    Json(payload): Json<CreateFieldPayload>,
) -> impl IntoResponse {
    let field_type = match payload.field_type.as_str() {
        "Number" => FieldType::Number,
        "Date" => FieldType::Date,
        "Boolean" => FieldType::Boolean,
        "Select" => FieldType::Select,
        "Relation" => FieldType::Relation,
        "Json" => FieldType::Json,
        _ => FieldType::Text,
    };

    match state
        .manage_metadata
        .create_field(object_id, payload.name, field_type, payload.settings)
        .await
    {
        Ok(field) => (StatusCode::CREATED, Json(field)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

pub async fn list_views_handler(
    State(state): State<MetadataAppState>,
    Query(query): Query<ListViewQuery>,
) -> impl IntoResponse {
    match state
        .manage_view
        .list_by_object(query.object_metadata_id)
        .await
    {
        Ok(views) => Json(views).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

pub async fn create_view_handler(
    State(state): State<MetadataAppState>,
    Json(payload): Json<CreateViewPayload>,
) -> impl IntoResponse {
    let view_type = match payload.view_type.as_str() {
        "Kanban" => ViewType::Kanban,
        "Calendar" => ViewType::Calendar,
        "List" => ViewType::List,
        _ => ViewType::Table,
    };

    match state
        .manage_view
        .create(
            payload.object_metadata_id,
            payload.name,
            view_type,
            payload.filters,
            payload.sort,
            Uuid::default(), // TODO: Get from auth
        )
        .await
    {
        Ok(view) => (StatusCode::CREATED, Json(view)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

pub async fn update_view_handler(
    State(state): State<MetadataAppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateViewPayload>,
) -> impl IntoResponse {
    match state
        .manage_view
        .update(id, payload.name, payload.filters, payload.sort)
        .await
    {
        Ok(view) => Json(view).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

pub async fn delete_view_handler(
    State(state): State<MetadataAppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.manage_view.delete(id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}
