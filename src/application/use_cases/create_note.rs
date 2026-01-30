use crate::application::ports::output::NoteRepository;
use crate::domain::{DomainError, Note};
use chrono::Utc;
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateNoteInput {
    pub title: String,
    pub body_v2: Option<String>,
}

pub struct CreateNote {
    note_repo: Arc<dyn NoteRepository>,
}

impl CreateNote {
    pub fn new(note_repo: Arc<dyn NoteRepository>) -> Self {
        Self { note_repo }
    }

    pub async fn execute(&self, input: CreateNoteInput) -> Result<Note, DomainError> {
        // Validate that title is not empty (INV-INT-002)
        if input.title.trim().is_empty() {
            return Err(DomainError::Validation("Title is required".to_string()));
        }

        let note = Note {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
            title: input.title,
            body_v2: input.body_v2,
            position: 0,
        };

        self.note_repo.create(note).await
    }
}
