pub use sea_orm_migration::prelude::*;

mod m20240130_000001_create_table;
mod m20240130_000002_create_users;
mod m20240130_000003_create_workspaces;

mod m20240130_000004_enhance_people;
mod m20240130_000005_create_companies;
mod m20240130_000006_create_email_system;
mod m20240130_000007_create_lead_system;
mod m20240130_000008_create_metadata_engine;
mod m20240130_000009_create_custom_object_data;
mod m20240130_000010_add_workspace_id;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240130_000001_create_table::InitialSchema),
            Box::new(m20240130_000002_create_users::CreateUsers),
            Box::new(m20240130_000003_create_workspaces::CreateWorkspaces),
            Box::new(m20240130_000004_enhance_people::EnhancePeople),
            Box::new(m20240130_000005_create_companies::CreateCompanies),
            Box::new(m20240130_000006_create_email_system::CreateEmailSystem),
            Box::new(m20240130_000007_create_lead_system::CreateLeadSystem),
            Box::new(m20240130_000008_create_metadata_engine::CreateMetadataEngine),
            Box::new(m20240130_000009_create_custom_object_data::Migration),
            Box::new(m20240130_000010_add_workspace_id::Migration),
        ]
    }
}
