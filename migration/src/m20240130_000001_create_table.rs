use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Person Table
        manager
            .create_table(
                Table::create()
                    .table(Person::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Person::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Person::Name).string().not_null())
                    .col(
                        ColumnDef::new(Person::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Person::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Opportunity Table
        manager
            .create_table(
                Table::create()
                    .table(Opportunity::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Opportunity::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Opportunity::Name).string().not_null())
                    .col(ColumnDef::new(Opportunity::Stage).string().not_null()) // Enum stored as string
                    .col(ColumnDef::new(Opportunity::AmountMicros).big_integer())
                    .col(
                        ColumnDef::new(Opportunity::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Opportunity::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Opportunity::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Person::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Person {
    Table,
    Id,
    Name,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Opportunity {
    Table,
    Id,
    Name,
    Stage,
    AmountMicros,
    CreatedAt,
    UpdatedAt,
}
