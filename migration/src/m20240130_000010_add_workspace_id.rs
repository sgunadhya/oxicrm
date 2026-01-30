use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Define the tables we are modifying
        let tables = vec![
            "leads",
            "companies",
            "emails",
            "object_metadata",
            "custom_object_data",
            "views",
        ];

        for table in tables {
            manager
                .alter_table(
                    Table::alter()
                        .table(Alias::new(table))
                        .add_column(
                            ColumnDef::new(Alias::new("workspace_id")).uuid().not_null(), // Enforce tenancy
                        )
                        .add_foreign_key(
                            TableForeignKey::new()
                                .name(format!("fk_{}_workspace_id", table))
                                .from_tbl(Alias::new(table))
                                .from_col(Alias::new("workspace_id"))
                                .to_tbl(Alias::new("workspaces"))
                                .to_col(Alias::new("id"))
                                .on_delete(ForeignKeyAction::Cascade)
                                .on_update(ForeignKeyAction::Cascade),
                        )
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let tables = vec![
            "leads",
            "companies",
            "emails",
            "object_metadata",
            "custom_object_data",
            "views",
        ];

        for table in tables {
            manager
                .alter_table(
                    Table::alter()
                        .table(Alias::new(table))
                        .drop_foreign_key(Alias::new(format!("fk_{}_workspace_id", table)))
                        .drop_column(Alias::new("workspace_id"))
                        .to_owned(),
                )
                .await?;
        }
        Ok(())
    }
}
