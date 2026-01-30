use crate::application::ports::output::LeadRepository;
use crate::application::use_cases::convert_lead::{ConvertLead, ConvertLeadInput};
use crate::application::use_cases::create_lead::{CreateLead, CreateLeadInput};
use crate::application::use_cases::manage_lead::ManageLead;
use crate::domain::states::{LeadSource, LeadStatus};
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
pub struct LeadAppState {
    pub create_lead: Arc<CreateLead>,
    pub manage_lead: Arc<ManageLead>,
    pub convert_lead: Arc<ConvertLead>,
    pub lead_repo: Arc<dyn LeadRepository>,
}

#[derive(Deserialize)]
pub struct CreateLeadPayload {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub company_name: Option<String>,
    pub job_title: Option<String>,
    pub source: String,
    pub notes: Option<String>,
}

#[derive(Deserialize)]
pub struct ConvertLeadPayload {
    pub create_person: bool,
    pub create_company: bool,
    pub create_opportunity: bool,
    pub opportunity_name: Option<String>,
    pub opportunity_amount: Option<i64>,
}

#[derive(Deserialize)]
pub struct UpdateLeadStatusPayload {
    pub status: String,
}

// POST /api/leads - Create lead
pub async fn create_lead_handler(
    State(state): State<LeadAppState>,
    Json(payload): Json<CreateLeadPayload>,
) -> impl IntoResponse {
    let source = match payload.source.as_str() {
        "web_form" => LeadSource::WebForm,
        "email" => LeadSource::Email,
        "referral" => LeadSource::Referral,
        _ => LeadSource::ManualEntry,
    };

    let input = CreateLeadInput {
        first_name: payload.first_name,
        last_name: payload.last_name,
        email: payload.email,
        phone: payload.phone,
        company_name: payload.company_name,
        job_title: payload.job_title,
        source,
        notes: payload.notes,
        workspace_id: Uuid::default(), // TODO: Get from auth context
    };

    match state.create_lead.execute(input).await {
        Ok(lead) => (StatusCode::CREATED, Json(lead)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

// GET /api/leads - List all leads
pub async fn list_leads_handler(State(state): State<LeadAppState>) -> impl IntoResponse {
    match state.manage_lead.list().await {
        Ok(leads) => Json(leads).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

// GET /api/leads/:id - Get specific lead
pub async fn get_lead_handler(
    State(state): State<LeadAppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.manage_lead.get(id).await {
        Ok(lead) => Json(lead).into_response(),
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "Lead not found" })),
        )
            .into_response(),
    }
}

// POST /api/leads/:id/convert - Convert lead
pub async fn convert_lead_handler(
    State(state): State<LeadAppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<ConvertLeadPayload>,
) -> impl IntoResponse {
    let input = ConvertLeadInput {
        lead_id: id,
        create_person: payload.create_person,
        create_company: payload.create_company,
        create_opportunity: payload.create_opportunity,
        opportunity_name: payload.opportunity_name,
        opportunity_amount: payload.opportunity_amount,
    };

    match state.convert_lead.execute(input).await {
        Ok(result) => Json(result).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

// PUT /api/leads/:id/status - Update lead status
pub async fn update_lead_status_handler(
    State(state): State<LeadAppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateLeadStatusPayload>,
) -> impl IntoResponse {
    let status = match payload.status.as_str() {
        "contacted" => LeadStatus::Contacted,
        "qualified" => LeadStatus::Qualified,
        "unqualified" => LeadStatus::Unqualified,
        "converted" => LeadStatus::Converted,
        _ => LeadStatus::New,
    };

    match state.manage_lead.update_status(id, status).await {
        Ok(lead) => Json(lead).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

// DELETE /api/leads/:id - Delete lead
pub async fn delete_lead_handler(
    State(state): State<LeadAppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.manage_lead.delete(id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

// POST /webhooks/lead-capture - Web form webhook
pub async fn lead_capture_webhook_handler(
    State(state): State<LeadAppState>,
    Json(payload): Json<CreateLeadPayload>,
) -> impl IntoResponse {
    // Same as create_lead_handler but always uses WebForm source
    let input = CreateLeadInput {
        first_name: payload.first_name,
        last_name: payload.last_name,
        email: payload.email,
        phone: payload.phone,
        company_name: payload.company_name,
        job_title: payload.job_title,
        source: LeadSource::WebForm,
        notes: payload.notes,
        workspace_id: Uuid::default(), // TODO: Get from auth context
    };

    match state.create_lead.execute(input).await {
        Ok(lead) => (StatusCode::CREATED, Json(lead)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}
