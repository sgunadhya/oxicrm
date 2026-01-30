pub use sea_orm_migration::prelude::*;

mod m20240130_000001_create_table;
mod m20240130_000002_create_users;
mod m20240130_000003_create_workspaces;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240130_000001_create_table::InitialSchema),
            Box::new(m20240130_000002_create_users::CreateUsers),
            Box::new(m20240130_000003_create_workspaces::CreateWorkspaces),
        ]
    }
}
