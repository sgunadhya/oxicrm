use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct CreateCompanies;

#[async_trait::async_trait]
impl MigrationTrait for CreateCompanies {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Company::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Company::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Company::Name).string().not_null())
                    .col(ColumnDef::new(Company::DomainName).string().not_null())
                    .col(ColumnDef::new(Company::Address).json()) // JSON or Text
                    .col(ColumnDef::new(Company::EmployeesCount).integer().default(0))
                    .col(
                        ColumnDef::new(Company::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Company::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Company::DeletedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await?;

        // Index on domain_name
        manager
            .create_index(
                Index::create()
                    .name("idx_company_domain_unique")
                    .table(Company::Table)
                    .col(Company::DomainName)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Company::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Company {
    Table,
    Id,
    Name,
    DomainName,
    Address,
    EmployeesCount,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
