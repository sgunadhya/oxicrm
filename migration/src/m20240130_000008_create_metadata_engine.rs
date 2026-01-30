use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct CreateMetadataEngine;

#[async_trait::async_trait]
impl MigrationTrait for CreateMetadataEngine {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // ObjectMetadata Table
        manager
            .create_table(
                Table::create()
                    .table(ObjectMetadata::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ObjectMetadata::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ObjectMetadata::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ObjectMetadata::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ObjectMetadata::NameSingular)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(ObjectMetadata::NamePlural)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ObjectMetadata::Description).string())
                    .to_owned(),
            )
            .await?;

        // FieldMetadata Table
        manager
            .create_table(
                Table::create()
                    .table(FieldMetadata::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(FieldMetadata::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(FieldMetadata::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(FieldMetadata::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(FieldMetadata::ObjectMetadataId)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(FieldMetadata::Name).string().not_null())
                    .col(ColumnDef::new(FieldMetadata::Type).string().not_null())
                    .col(
                        ColumnDef::new(FieldMetadata::IsCustom)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(ColumnDef::new(FieldMetadata::Settings).json())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_field_object")
                            .from(FieldMetadata::Table, FieldMetadata::ObjectMetadataId)
                            .to(ObjectMetadata::Table, ObjectMetadata::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // View Table
        manager
            .create_table(
                Table::create()
                    .table(View::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(View::Id).uuid().not_null().primary_key())
                    .col(
                        ColumnDef::new(View::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(View::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(View::ObjectMetadataId).uuid().not_null())
                    .col(ColumnDef::new(View::Name).string().not_null())
                    .col(ColumnDef::new(View::Type).string().not_null())
                    .col(ColumnDef::new(View::Filters).json().not_null())
                    .col(ColumnDef::new(View::Sort).json().not_null())
                    .col(
                        ColumnDef::new(View::Position)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_view_object")
                            .from(View::Table, View::ObjectMetadataId)
                            .to(ObjectMetadata::Table, ObjectMetadata::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Index on field name + object id
        manager
            .create_index(
                Index::create()
                    .name("idx_field_name_object")
                    .table(FieldMetadata::Table)
                    .col(FieldMetadata::ObjectMetadataId)
                    .col(FieldMetadata::Name)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(View::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(FieldMetadata::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(ObjectMetadata::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum ObjectMetadata {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    NameSingular,
    NamePlural,
    Description,
}

#[derive(Iden)]
enum FieldMetadata {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    ObjectMetadataId,
    Name,
    Type,
    IsCustom,
    Settings,
}

#[derive(Iden)]
enum View {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    ObjectMetadataId,
    Name,
    Type,
    Filters,
    Sort,
    Position,
}
