use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CustomObjectData::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CustomObjectData::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(CustomObjectData::ObjectMetadataId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CustomObjectData::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CustomObjectData::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CustomObjectData::Properties)
                            .json()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_custom_object_data_object_metadata_id")
                            .from(CustomObjectData::Table, CustomObjectData::ObjectMetadataId)
                            .to(ObjectMetadata::Table, ObjectMetadata::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CustomObjectData::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum CustomObjectData {
    Table,
    Id,
    ObjectMetadataId,
    CreatedAt,
    UpdatedAt,
    Properties,
}

#[derive(DeriveIden)]
enum ObjectMetadata {
    Table,
    Id,
}
