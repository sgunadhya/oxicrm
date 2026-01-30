use crate::application::ports::output::MetadataRepository;
use crate::domain::metadata::{FieldMetadata, ObjectMetadata};
use crate::domain::DomainError;
use std::sync::Arc;
use uuid::Uuid;

pub struct ManageMetadata {
    metadata_repo: Arc<dyn MetadataRepository>,
}

impl ManageMetadata {
    pub fn new(metadata_repo: Arc<dyn MetadataRepository>) -> Self {
        Self { metadata_repo }
    }

    pub async fn get_schema(
        &self,
    ) -> Result<Vec<(ObjectMetadata, Vec<FieldMetadata>)>, DomainError> {
        self.metadata_repo.get_schema().await
    }

    pub async fn create_object(
        &self,
        name_singular: String,
        name_plural: String,
        description: Option<String>,
        workspace_id: Uuid,
    ) -> Result<ObjectMetadata, DomainError> {
        // Check for existing
        if (self
            .metadata_repo
            .find_object_by_name(&name_singular)
            .await?)
            .is_some()
        {
            return Err(DomainError::Validation(format!(
                "Object with name {} already exists",
                name_singular
            )));
        }

        let object = ObjectMetadata {
            id: Uuid::new_v4(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            name_singular,
            name_plural,
            description,
            workspace_id,
        };

        self.metadata_repo.create_object(object).await
    }

    pub async fn create_field(
        &self,
        object_metadata_id: Uuid,
        name: String,
        field_type: crate::domain::metadata::FieldType,
        settings: Option<serde_json::Value>,
    ) -> Result<FieldMetadata, DomainError> {
        let field = FieldMetadata {
            id: Uuid::new_v4(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            object_metadata_id,
            name,
            field_type,
            is_custom: true,
            settings,
        };

        self.metadata_repo.create_field(field).await
    }
}
