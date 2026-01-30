use crate::application::ports::output::ViewRepository;
use crate::domain::metadata::{View, ViewType};
use crate::domain::DomainError;
use std::sync::Arc;
use uuid::Uuid;

pub struct ManageView {
    view_repo: Arc<dyn ViewRepository>,
}

impl ManageView {
    pub fn new(view_repo: Arc<dyn ViewRepository>) -> Self {
        Self { view_repo }
    }

    pub async fn list_by_object(&self, object_metadata_id: Uuid) -> Result<Vec<View>, DomainError> {
        self.view_repo.find_by_object(object_metadata_id).await
    }

    pub async fn get(&self, id: Uuid) -> Result<View, DomainError> {
        self.view_repo
            .find_by_id(id)
            .await?
            .ok_or(DomainError::NotFound)
    }

    pub async fn create(
        &self,
        object_metadata_id: Uuid,
        name: String,
        view_type: ViewType,
        filters: serde_json::Value,
        sort: serde_json::Value,
        workspace_id: Uuid,
    ) -> Result<View, DomainError> {
        let view = View {
            id: Uuid::new_v4(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            object_metadata_id,
            name,
            view_type,
            filters,
            sort,
            position: 0,
            workspace_id,
        };

        self.view_repo.create(view).await
    }

    pub async fn update(
        &self,
        id: Uuid,
        name: Option<String>,
        filters: Option<serde_json::Value>,
        sort: Option<serde_json::Value>,
    ) -> Result<View, DomainError> {
        let mut view = self.get(id).await?;

        if let Some(n) = name {
            view.name = n;
        }
        if let Some(f) = filters {
            view.filters = f;
        }
        if let Some(s) = sort {
            view.sort = s;
        }

        view.updated_at = chrono::Utc::now();
        self.view_repo.update(view).await
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        self.view_repo.delete(id).await
    }
}
