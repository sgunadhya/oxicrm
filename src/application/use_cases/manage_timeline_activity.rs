use crate::application::ports::output::TimelineActivityRepository;
use crate::domain::{DomainError, TimelineActivity};
use std::sync::Arc;
use uuid::Uuid;

pub struct ManageTimelineActivity {
    activity_repo: Arc<dyn TimelineActivityRepository>,
}

impl ManageTimelineActivity {
    pub fn new(activity_repo: Arc<dyn TimelineActivityRepository>) -> Self {
        Self { activity_repo }
    }

    pub async fn get_all(&self) -> Result<Vec<TimelineActivity>, DomainError> {
        self.activity_repo.find_all().await
    }

    pub async fn get_by_person_id(&self, person_id: Uuid) -> Result<Vec<TimelineActivity>, DomainError> {
        self.activity_repo.find_by_person_id(person_id).await
    }

    pub async fn get_by_company_id(&self, company_id: Uuid) -> Result<Vec<TimelineActivity>, DomainError> {
        self.activity_repo.find_by_company_id(company_id).await
    }

    pub async fn get_by_opportunity_id(&self, opportunity_id: Uuid) -> Result<Vec<TimelineActivity>, DomainError> {
        self.activity_repo.find_by_opportunity_id(opportunity_id).await
    }

    pub async fn get_by_task_id(&self, task_id: Uuid) -> Result<Vec<TimelineActivity>, DomainError> {
        self.activity_repo.find_by_task_id(task_id).await
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        self.activity_repo.delete(id).await
    }
}
