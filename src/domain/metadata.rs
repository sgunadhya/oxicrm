use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectMetadata {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name_singular: String,
    pub name_plural: String,
    pub description: Option<String>,
    pub workspace_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FieldType {
    Text,
    Number,
    Date,
    Boolean,
    Select,
    Relation,
    Json,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldMetadata {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub object_metadata_id: Uuid,
    pub name: String,
    pub field_type: FieldType,
    pub is_custom: bool,
    pub settings: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ViewType {
    Table,
    Kanban,
    Calendar,
    List,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct View {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub object_metadata_id: Uuid,
    pub name: String,
    pub view_type: ViewType,
    pub filters: serde_json::Value,
    pub sort: serde_json::Value,
    pub position: i32,
    pub workspace_id: Uuid,
}
