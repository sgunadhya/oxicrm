use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct CreateEmailSystem;

#[async_trait::async_trait]
impl MigrationTrait for CreateEmailSystem {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // EmailTemplate Table
        manager
            .create_table(
                Table::create()
                    .table(EmailTemplate::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(EmailTemplate::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(EmailTemplate::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(EmailTemplate::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(EmailTemplate::Name).string().not_null())
                    .col(ColumnDef::new(EmailTemplate::Subject).string().not_null())
                    .col(ColumnDef::new(EmailTemplate::BodyText).text().not_null())
                    .col(ColumnDef::new(EmailTemplate::BodyHtml).text())
                    .col(
                        ColumnDef::new(EmailTemplate::Category)
                            .string()
                            .not_null()
                            .default("manual"),
                    )
                    .to_owned(),
            )
            .await?;

        // Index on email template name (unique)
        manager
            .create_index(
                Index::create()
                    .name("idx_email_template_name_unique")
                    .table(EmailTemplate::Table)
                    .col(EmailTemplate::Name)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // Email Table
        manager
            .create_table(
                Table::create()
                    .table(Email::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Email::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Email::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Email::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Email::Direction)
                            .string()
                            .not_null()
                            .default("outbound"),
                    )
                    .col(
                        ColumnDef::new(Email::Status)
                            .string()
                            .not_null()
                            .default("pending"),
                    )
                    .col(ColumnDef::new(Email::FromEmail).string().not_null())
                    .col(ColumnDef::new(Email::ToEmail).string().not_null())
                    .col(ColumnDef::new(Email::CcEmails).json())
                    .col(ColumnDef::new(Email::BccEmails).json())
                    .col(ColumnDef::new(Email::Subject).string().not_null())
                    .col(ColumnDef::new(Email::BodyText).text().not_null())
                    .col(ColumnDef::new(Email::BodyHtml).text())
                    .col(ColumnDef::new(Email::SentAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Email::FailedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Email::ErrorMessage).text())
                    .col(ColumnDef::new(Email::EmailTemplateId).uuid())
                    .col(ColumnDef::new(Email::TimelineActivityId).uuid())
                    // Polymorphic relationships
                    .col(ColumnDef::new(Email::PersonId).uuid())
                    .col(ColumnDef::new(Email::CompanyId).uuid())
                    .col(ColumnDef::new(Email::OpportunityId).uuid())
                    .col(ColumnDef::new(Email::TaskId).uuid())
                    .col(ColumnDef::new(Email::WorkflowId).uuid())
                    .col(ColumnDef::new(Email::WorkflowRunId).uuid())
                    .col(ColumnDef::new(Email::Metadata).json())
                    .to_owned(),
            )
            .await?;

        // Index on status for finding pending emails
        manager
            .create_index(
                Index::create()
                    .name("idx_email_status")
                    .table(Email::Table)
                    .col(Email::Status)
                    .to_owned(),
            )
            .await?;

        // Index on direction
        manager
            .create_index(
                Index::create()
                    .name("idx_email_direction")
                    .table(Email::Table)
                    .col(Email::Direction)
                    .to_owned(),
            )
            .await?;

        // Index on person_id for quick lookups
        manager
            .create_index(
                Index::create()
                    .name("idx_email_person_id")
                    .table(Email::Table)
                    .col(Email::PersonId)
                    .to_owned(),
            )
            .await?;

        // Index on company_id
        manager
            .create_index(
                Index::create()
                    .name("idx_email_company_id")
                    .table(Email::Table)
                    .col(Email::CompanyId)
                    .to_owned(),
            )
            .await?;

        // Index on opportunity_id
        manager
            .create_index(
                Index::create()
                    .name("idx_email_opportunity_id")
                    .table(Email::Table)
                    .col(Email::OpportunityId)
                    .to_owned(),
            )
            .await?;

        // Index on workflow_run_id
        manager
            .create_index(
                Index::create()
                    .name("idx_email_workflow_run_id")
                    .table(Email::Table)
                    .col(Email::WorkflowRunId)
                    .to_owned(),
            )
            .await

    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Email::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(EmailTemplate::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum EmailTemplate {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    Name,
    Subject,
    BodyText,
    BodyHtml,
    Category,
}

#[derive(Iden)]
enum Email {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    Direction,
    Status,
    FromEmail,
    ToEmail,
    CcEmails,
    BccEmails,
    Subject,
    BodyText,
    BodyHtml,
    SentAt,
    FailedAt,
    ErrorMessage,
    EmailTemplateId,
    TimelineActivityId,
    PersonId,
    CompanyId,
    OpportunityId,
    TaskId,
    WorkflowId,
    WorkflowRunId,
    Metadata,
}
