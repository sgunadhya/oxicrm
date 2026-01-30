use crate::application::ports::output::NoteRepository;
use crate::domain::{DomainError, Note};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct UpdateNoteInput {
    pub id: Uuid,
    pub title: Option<String>,
    pub body_v2: Option<String>,
}

pub struct ManageNote {
    note_repo: Arc<dyn NoteRepository>,
}

impl ManageNote {
    pub fn new(note_repo: Arc<dyn NoteRepository>) -> Self {
        Self { note_repo }
    }

    pub async fn get_all(&self) -> Result<Vec<Note>, DomainError> {
        self.note_repo.find_all().await
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<Note>, DomainError> {
        self.note_repo.find_by_id(id).await
    }

    pub async fn update(&self, input: UpdateNoteInput) -> Result<Note, DomainError> {
        // First fetch the existing note
        let existing = self
            .note_repo
            .find_by_id(input.id)
            .await?
            .ok_or(DomainError::NotFound)?;

        // Validate title if provided (INV-INT-002)
        if let Some(ref title) = input.title {
            if title.trim().is_empty() {
                return Err(DomainError::Validation("Title is required".to_string()));
            }
        }

        // Build updated note
        let updated = Note {
            id: existing.id,
            created_at: existing.created_at,
            updated_at: chrono::Utc::now(),
            deleted_at: existing.deleted_at,
            title: input.title.unwrap_or(existing.title),
            body_v2: input.body_v2.or(existing.body_v2),
            position: existing.position,
            workspace_id: existing.workspace_id,
        };

        self.note_repo.update(updated).await
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        self.note_repo.delete(id).await
    }
}
