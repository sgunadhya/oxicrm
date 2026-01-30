use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct EnhancePeople;

#[async_trait::async_trait]
impl MigrationTrait for EnhancePeople {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add columns separately for SQLite compatibility
        manager
            .alter_table(
                Table::alter()
                    .table(Person::Table)
                    .add_column(
                        ColumnDef::new(Person::Email)
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Person::Table)
                    .add_column(ColumnDef::new(Person::DeletedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Person::Table)
                    .add_column(ColumnDef::new(Person::CompanyId).uuid())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Person::Table)
                    .add_column(ColumnDef::new(Person::Position).integer().default(0))
                    .to_owned(),
            )
            .await?;

        // Add index for unique email
        manager
            .create_index(
                Index::create()
                    .name("idx_person_email_unique")
                    .table(Person::Table)
                    .col(Person::Email)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_person_email_unique")
                    .table(Person::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Person::Table)
                    .drop_column(Person::Email)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Person::Table)
                    .drop_column(Person::DeletedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Person::Table)
                    .drop_column(Person::CompanyId)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Person::Table)
                    .drop_column(Person::Position)
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
enum Person {
    Table,
    Email,
    DeletedAt,
    CompanyId,
    Position,
}
