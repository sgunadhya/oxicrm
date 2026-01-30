use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct CreateLeadSystem;

#[async_trait::async_trait]
impl MigrationTrait for CreateLeadSystem {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Lead Table
        manager
            .create_table(
                Table::create()
                    .table(Lead::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Lead::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Lead::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Lead::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Lead::DeletedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Lead::FirstName).string().not_null())
                    .col(ColumnDef::new(Lead::LastName).string().not_null())
                    .col(ColumnDef::new(Lead::Email).string().not_null())
                    .col(ColumnDef::new(Lead::Phone).string())
                    .col(ColumnDef::new(Lead::CompanyName).string())
                    .col(ColumnDef::new(Lead::JobTitle).string())
                    .col(
                        ColumnDef::new(Lead::Source)
                            .string()
                            .not_null()
                            .default("manual_entry"),
                    )
                    .col(
                        ColumnDef::new(Lead::Status)
                            .string()
                            .not_null()
                            .default("new"),
                    )
                    .col(
                        ColumnDef::new(Lead::Score)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(Lead::Notes).text())
                    .col(
                        ColumnDef::new(Lead::Position)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(Lead::AssignedToId).uuid())
                    .col(ColumnDef::new(Lead::ConvertedPersonId).uuid())
                    .col(ColumnDef::new(Lead::ConvertedCompanyId).uuid())
                    .col(ColumnDef::new(Lead::ConvertedOpportunityId).uuid())
                    .col(ColumnDef::new(Lead::ConvertedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Lead::LastContactedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await?;

        // Unique index on email
        manager
            .create_index(
                Index::create()
                    .name("idx_lead_email_unique")
                    .table(Lead::Table)
                    .col(Lead::Email)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // Index on status for filtering
        manager
            .create_index(
                Index::create()
                    .name("idx_lead_status")
                    .table(Lead::Table)
                    .col(Lead::Status)
                    .to_owned(),
            )
            .await?;

        // Index on source for reporting
        manager
            .create_index(
                Index::create()
                    .name("idx_lead_source")
                    .table(Lead::Table)
                    .col(Lead::Source)
                    .to_owned(),
            )
            .await?;

        // Index on assigned_to_id for finding assigned leads
        manager
            .create_index(
                Index::create()
                    .name("idx_lead_assigned_to")
                    .table(Lead::Table)
                    .col(Lead::AssignedToId)
                    .to_owned(),
            )
            .await?;

        // Index on score for sorting by priority (descending)
        manager
            .create_index(
                Index::create()
                    .name("idx_lead_score")
                    .table(Lead::Table)
                    .col(Lead::Score)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Lead::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Lead {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
    FirstName,
    LastName,
    Email,
    Phone,
    CompanyName,
    JobTitle,
    Source,
    Status,
    Score,
    Notes,
    Position,
    AssignedToId,
    ConvertedPersonId,
    ConvertedCompanyId,
    ConvertedOpportunityId,
    ConvertedAt,
    LastContactedAt,
}
