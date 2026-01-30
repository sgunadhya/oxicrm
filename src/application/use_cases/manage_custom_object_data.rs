use crate::application::ports::output::{CustomObjectDataRepository, MetadataRepository};
use crate::domain::custom_object_data::CustomObjectData;
use crate::domain::DomainError;
use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

pub struct ManageCustomObjectData {
    repo: Arc<dyn CustomObjectDataRepository>,
    metadata_repo: Arc<dyn MetadataRepository>,
}

impl ManageCustomObjectData {
    pub fn new(
        repo: Arc<dyn CustomObjectDataRepository>,
        metadata_repo: Arc<dyn MetadataRepository>,
    ) -> Self {
        Self {
            repo,
            metadata_repo,
        }
    }

    pub async fn create_record(
        &self,
        object_metadata_id: Uuid,
        properties: serde_json::Value,
        workspace_id: Uuid,
    ) -> Result<CustomObjectData, DomainError> {
        // Verify object exists
        let object = self
            .metadata_repo
            .find_object_by_id(object_metadata_id)
            .await?;

        if object.is_none() {
            return Err(DomainError::Validation("Object not found".to_string()));
        }

        let record = CustomObjectData {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            object_metadata_id,
            properties,
            workspace_id,
        };

        self.repo.create(record).await
    }

    pub async fn update_record(
        &self,
        id: Uuid,
        properties: serde_json::Value,
    ) -> Result<CustomObjectData, DomainError> {
        let existing = self.repo.find_by_id(id).await?;

        if let Some(mut record) = existing {
            record.properties = properties;
            record.updated_at = Utc::now();
            self.repo.update(record).await
        } else {
            Err(DomainError::Validation("Record not found".to_string()))
        }
    }

    pub async fn delete_record(&self, id: Uuid) -> Result<(), DomainError> {
        self.repo.delete(id).await
    }

    pub async fn get_record(&self, id: Uuid) -> Result<Option<CustomObjectData>, DomainError> {
        self.repo.find_by_id(id).await
    }

    pub async fn list_records(
        &self,
        object_metadata_id: Uuid,
    ) -> Result<Vec<CustomObjectData>, DomainError> {
        self.repo
            .find_by_object_metadata_id(object_metadata_id)
            .await
    }
}
