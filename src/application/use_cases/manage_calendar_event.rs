use crate::application::ports::output::CalendarEventRepository;
use crate::domain::{CalendarEvent, DomainError};
use std::sync::Arc;
use uuid::Uuid;

pub struct ManageCalendarEvent {
    calendar_event_repo: Arc<dyn CalendarEventRepository>,
}

impl ManageCalendarEvent {
    pub fn new(calendar_event_repo: Arc<dyn CalendarEventRepository>) -> Self {
        Self { calendar_event_repo }
    }

    pub async fn get_all(&self) -> Result<Vec<CalendarEvent>, DomainError> {
        self.calendar_event_repo.find_all().await
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<CalendarEvent>, DomainError> {
        self.calendar_event_repo.find_by_id(id).await
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        self.calendar_event_repo.delete(id).await
    }
}
