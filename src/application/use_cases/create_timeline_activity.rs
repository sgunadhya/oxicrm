use crate::application::ports::output::TimelineActivityRepository;
use crate::domain::{DomainError, TimelineActivity};
use chrono::Utc;
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateTimelineActivityInput {
    pub name: String,
    pub workspace_member_id: Option<Uuid>,
    pub person_id: Option<Uuid>,
    pub company_id: Option<Uuid>,
    pub opportunity_id: Option<Uuid>,
    pub task_id: Option<Uuid>,
    pub note_id: Option<Uuid>,
    pub calendar_event_id: Option<Uuid>,
    pub workflow_id: Option<Uuid>,
}

pub struct CreateTimelineActivity {
    activity_repo: Arc<dyn TimelineActivityRepository>,
}

impl CreateTimelineActivity {
    pub fn new(activity_repo: Arc<dyn TimelineActivityRepository>) -> Self {
        Self { activity_repo }
    }

    pub async fn execute(&self, input: CreateTimelineActivityInput) -> Result<TimelineActivity, DomainError> {
        let activity = TimelineActivity {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            name: input.name,
            workspace_member_id: input.workspace_member_id,
            person_id: input.person_id,
            company_id: input.company_id,
            opportunity_id: input.opportunity_id,
            task_id: input.task_id,
            note_id: input.note_id,
            calendar_event_id: input.calendar_event_id,
            workflow_id: input.workflow_id,
        };

        self.activity_repo.create(activity).await
    }
}
