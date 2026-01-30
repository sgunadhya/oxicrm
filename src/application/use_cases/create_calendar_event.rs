use crate::application::ports::output::CalendarEventRepository;
use crate::domain::{CalendarEvent, DomainError};
use chrono::{DateTime, Utc};
use std::sync::Arc;
use uuid::Uuid;

pub struct CreateCalendarEventInput {
    pub connected_account_id: Uuid,
    pub title: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub description: Option<String>,
}

pub struct CreateCalendarEvent {
    calendar_event_repo: Arc<dyn CalendarEventRepository>,
}

impl CreateCalendarEvent {
    pub fn new(calendar_event_repo: Arc<dyn CalendarEventRepository>) -> Self {
        Self { calendar_event_repo }
    }

    pub async fn execute(&self, input: CreateCalendarEventInput) -> Result<CalendarEvent, DomainError> {
        let event = CalendarEvent {
            id: Uuid::new_v4(),
            connected_account_id: input.connected_account_id,
            title: input.title,
            start_time: input.start_time,
            end_time: input.end_time,
            description: input.description,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.calendar_event_repo.create(event).await
    }
}
