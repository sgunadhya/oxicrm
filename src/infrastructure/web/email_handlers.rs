use crate::application::ports::output::{EmailRepository, EmailTemplateRepository};
use crate::application::use_cases::manage_email_template::{
    CreateEmailTemplateInput, ManageEmailTemplate, UpdateEmailTemplateInput,
};
use crate::application::use_cases::receive_email::{ReceiveEmail, ReceiveEmailInput};
use crate::application::use_cases::send_email::{SendEmail, SendEmailInput};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct EmailAppState {
    pub send_email: Arc<SendEmail>,
    pub receive_email: Arc<ReceiveEmail>,
    pub manage_email_template: Arc<ManageEmailTemplate>,
    pub email_repo: Arc<dyn EmailRepository>,
    pub email_template_repo: Arc<dyn EmailTemplateRepository>,
}

#[derive(Deserialize)]
pub struct SendEmailPayload {
    pub from_email: String,
    pub to_email: String,
    pub cc_emails: Option<Vec<String>>,
    pub bcc_emails: Option<Vec<String>>,
    pub subject: String,
    pub body_text: String,
    pub body_html: Option<String>,
    pub template_id: Option<Uuid>,
    pub template_variables: Option<serde_json::Value>,
    pub person_id: Option<Uuid>,
    pub company_id: Option<Uuid>,
    pub opportunity_id: Option<Uuid>,
    pub task_id: Option<Uuid>,
    pub workflow_id: Option<Uuid>,
    pub workflow_run_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct InboundEmailWebhookPayload {
    pub from_email: String,
    pub to_email: String,
    pub subject: String,
    pub body_text: String,
    pub body_html: Option<String>,
    pub received_at: Option<DateTime<Utc>>,
}

#[derive(Serialize)]
pub struct EmailResponse {
    pub id: Uuid,
    pub status: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct EmailTemplateResponse {
    pub id: Uuid,
    pub name: String,
    pub subject: String,
    pub category: String,
}

// POST /api/emails - Send an email
pub async fn send_email_handler(
    State(state): State<EmailAppState>,
    Json(payload): Json<SendEmailPayload>,
) -> impl IntoResponse {
    let input = SendEmailInput {
        from_email: payload.from_email,
        to_email: payload.to_email,
        cc_emails: payload.cc_emails,
        bcc_emails: payload.bcc_emails,
        subject: payload.subject,
        body_text: payload.body_text,
        body_html: payload.body_html,
        template_id: payload.template_id,
        template_variables: payload.template_variables,
        person_id: payload.person_id,
        company_id: payload.company_id,
        opportunity_id: payload.opportunity_id,
        task_id: payload.task_id,
        workflow_id: payload.workflow_id,
        workflow_run_id: payload.workflow_run_id,
    };

    match state.send_email.execute(input).await {
        Ok(email) => {
            let status_str = match email.status {
                crate::domain::EmailStatus::Pending => "pending",
                crate::domain::EmailStatus::Sent => "sent",
                crate::domain::EmailStatus::Failed => "failed",
                crate::domain::EmailStatus::Received => "received",
            };

            let response = EmailResponse {
                id: email.id,
                status: status_str.to_string(),
                message: "Email processed successfully".to_string(),
            };

            (StatusCode::CREATED, Json(response)).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to send email: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to send email: {}", e)
                })),
            )
                .into_response()
        }
    }
}

// POST /webhooks/inbound-email - Receive inbound email webhook
pub async fn inbound_email_webhook_handler(
    State(state): State<EmailAppState>,
    Json(payload): Json<InboundEmailWebhookPayload>,
) -> impl IntoResponse {
    let input = ReceiveEmailInput {
        from_email: payload.from_email,
        to_email: payload.to_email,
        subject: payload.subject,
        body_text: payload.body_text,
        body_html: payload.body_html,
        received_at: payload.received_at.unwrap_or_else(Utc::now),
    };

    match state.receive_email.execute(input).await {
        Ok(email) => {
            let response = EmailResponse {
                id: email.id,
                status: "received".to_string(),
                message: "Inbound email processed successfully".to_string(),
            };

            (StatusCode::CREATED, Json(response)).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to process inbound email: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to process inbound email: {}", e)
                })),
            )
                .into_response()
        }
    }
}

// GET /api/email-templates - List all email templates
pub async fn list_email_templates_handler(
    State(state): State<EmailAppState>,
) -> impl IntoResponse {
    match state.manage_email_template.list().await {
        Ok(templates) => {
            let response: Vec<EmailTemplateResponse> = templates
                .into_iter()
                .map(|t| EmailTemplateResponse {
                    id: t.id,
                    name: t.name,
                    subject: t.subject,
                    category: t.category,
                })
                .collect();

            Json(response).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to list email templates: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to list email templates: {}", e)
                })),
            )
                .into_response()
        }
    }
}

// POST /api/email-templates - Create email template
pub async fn create_email_template_handler(
    State(state): State<EmailAppState>,
    Json(input): Json<CreateEmailTemplateInput>,
) -> impl IntoResponse {
    match state.manage_email_template.create(input).await {
        Ok(template) => {
            let response = EmailTemplateResponse {
                id: template.id,
                name: template.name,
                subject: template.subject,
                category: template.category,
            };

            (StatusCode::CREATED, Json(response)).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to create email template: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to create email template: {}", e)
                })),
            )
                .into_response()
        }
    }
}

// GET /api/email-templates/:id - Get email template
pub async fn get_email_template_handler(
    State(state): State<EmailAppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.manage_email_template.get(id).await {
        Ok(template) => Json(template).into_response(),
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "Email template not found"
            })),
        )
            .into_response(),
    }
}

// PUT /api/email-templates/:id - Update email template
pub async fn update_email_template_handler(
    State(state): State<EmailAppState>,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateEmailTemplateInput>,
) -> impl IntoResponse {
    match state.manage_email_template.update(id, input).await {
        Ok(template) => Json(template).into_response(),
        Err(e) => {
            tracing::error!("Failed to update email template: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to update email template: {}", e)
                })),
            )
                .into_response()
        }
    }
}

// DELETE /api/email-templates/:id - Delete email template
pub async fn delete_email_template_handler(
    State(state): State<EmailAppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.manage_email_template.delete(id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => {
            tracing::error!("Failed to delete email template: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to delete email template: {}", e)
                })),
            )
                .into_response()
        }
    }
}

// GET /api/emails - List all emails
pub async fn list_emails_handler(State(state): State<EmailAppState>) -> impl IntoResponse {
    match state.email_repo.find_all().await {
        Ok(emails) => Json(emails).into_response(),
        Err(e) => {
            tracing::error!("Failed to list emails: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to list emails: {}", e)
                })),
            )
                .into_response()
        }
    }
}

// GET /api/emails/:id - Get email by ID
pub async fn get_email_handler(
    State(state): State<EmailAppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.email_repo.find_by_id(id).await {
        Ok(Some(email)) => Json(email).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "Email not found"
            })),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("Failed to get email: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to get email: {}", e)
                })),
            )
                .into_response()
        }
    }
}
