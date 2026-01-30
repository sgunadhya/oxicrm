use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct InitialSchema;

#[async_trait::async_trait]
impl MigrationTrait for InitialSchema {
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
                    .col(ColumnDef::new(Opportunity::Stage).string().not_null())
                    .col(ColumnDef::new(Opportunity::AmountMicros).big_integer())
                    .col(
                        ColumnDef::new(Opportunity::CurrencyCode)
                            .string()
                            .default("USD"),
                    )
                    .col(ColumnDef::new(Opportunity::CloseDate).date())
                    .col(ColumnDef::new(Opportunity::CompanyId).uuid())
                    .col(ColumnDef::new(Opportunity::PointOfContactId).uuid())
                    .col(ColumnDef::new(Opportunity::OwnerId).uuid())
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
                    .col(ColumnDef::new(Opportunity::DeletedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await?;

        // Task Table
        manager
            .create_table(
                Table::create()
                    .table(Task::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Task::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Task::Title).string().not_null())
                    .col(ColumnDef::new(Task::Body).text())
                    .col(ColumnDef::new(Task::Status).string().not_null().default("TODO"))
                    .col(ColumnDef::new(Task::Position).integer().not_null().default(0))
                    .col(ColumnDef::new(Task::AssigneeId).uuid())
                    .col(ColumnDef::new(Task::DueAt).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(Task::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Task::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Task::DeletedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await?;

        // Note Table
        manager
            .create_table(
                Table::create()
                    .table(Note::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Note::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Note::Title).string().not_null())
                    .col(ColumnDef::new(Note::BodyV2).text())
                    .col(ColumnDef::new(Note::Position).integer().not_null().default(0))
                    .col(
                        ColumnDef::new(Note::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Note::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Note::DeletedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await?;

        // TaskTarget Table (polymorphic join table)
        manager
            .create_table(
                Table::create()
                    .table(TaskTarget::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TaskTarget::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TaskTarget::TaskId).uuid().not_null())
                    .col(ColumnDef::new(TaskTarget::PersonId).uuid())
                    .col(ColumnDef::new(TaskTarget::CompanyId).uuid())
                    .col(ColumnDef::new(TaskTarget::OpportunityId).uuid())
                    .col(
                        ColumnDef::new(TaskTarget::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Workflow Table
        manager
            .create_table(
                Table::create()
                    .table(Workflow::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Workflow::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Workflow::Name).string().not_null())
                    .col(ColumnDef::new(Workflow::LastPublishedVersionId).uuid())
                    .col(
                        ColumnDef::new(Workflow::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Workflow::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // WorkflowVersion Table
        manager
            .create_table(
                Table::create()
                    .table(WorkflowVersion::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(WorkflowVersion::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(WorkflowVersion::WorkflowId).uuid().not_null())
                    .col(
                        ColumnDef::new(WorkflowVersion::Status)
                            .string()
                            .not_null()
                            .default("DRAFT"),
                    )
                    .col(
                        ColumnDef::new(WorkflowVersion::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(WorkflowVersion::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // WorkflowVersionStep Table
        manager
            .create_table(
                Table::create()
                    .table(WorkflowVersionStep::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(WorkflowVersionStep::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(WorkflowVersionStep::WorkflowVersionId).uuid().not_null())
                    .col(ColumnDef::new(WorkflowVersionStep::Type).string().not_null())
                    .col(ColumnDef::new(WorkflowVersionStep::Settings).json())
                    .col(ColumnDef::new(WorkflowVersionStep::Position).integer().not_null().default(0))
                    .col(
                        ColumnDef::new(WorkflowVersionStep::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // WorkflowRun Table
        manager
            .create_table(
                Table::create()
                    .table(WorkflowRun::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(WorkflowRun::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(WorkflowRun::WorkflowVersionId).uuid().not_null())
                    .col(
                        ColumnDef::new(WorkflowRun::Status)
                            .string()
                            .not_null()
                            .default("RUNNING"),
                    )
                    .col(ColumnDef::new(WorkflowRun::Output).json())
                    .col(ColumnDef::new(WorkflowRun::Error).text())
                    .col(
                        ColumnDef::new(WorkflowRun::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(WorkflowRun::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // ConnectedAccount Table
        manager
            .create_table(
                Table::create()
                    .table(ConnectedAccount::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ConnectedAccount::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ConnectedAccount::Provider).string().not_null())
                    .col(ColumnDef::new(ConnectedAccount::AccountOwnerId).uuid().not_null())
                    .col(
                        ColumnDef::new(ConnectedAccount::Status)
                            .string()
                            .not_null()
                            .default("CONNECTED"),
                    )
                    .col(
                        ColumnDef::new(ConnectedAccount::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ConnectedAccount::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // CalendarEvent Table
        manager
            .create_table(
                Table::create()
                    .table(CalendarEvent::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CalendarEvent::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(CalendarEvent::ConnectedAccountId).uuid().not_null())
                    .col(ColumnDef::new(CalendarEvent::Title).string().not_null())
                    .col(ColumnDef::new(CalendarEvent::StartTime).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(CalendarEvent::EndTime).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(CalendarEvent::Description).text())
                    .col(
                        ColumnDef::new(CalendarEvent::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CalendarEvent::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // CalendarEventParticipant Table
        manager
            .create_table(
                Table::create()
                    .table(CalendarEventParticipant::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CalendarEventParticipant::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(CalendarEventParticipant::CalendarEventId).uuid().not_null())
                    .col(ColumnDef::new(CalendarEventParticipant::Email).string().not_null())
                    .col(ColumnDef::new(CalendarEventParticipant::PersonId).uuid())
                    .col(
                        ColumnDef::new(CalendarEventParticipant::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CalendarEventParticipant::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(CalendarEvent::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(ConnectedAccount::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(WorkflowRun::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(WorkflowVersionStep::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(WorkflowVersion::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Workflow::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(TaskTarget::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Note::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Task::Table).to_owned())
            .await?;
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
    CurrencyCode,
    CloseDate,
    CompanyId,
    PointOfContactId,
    OwnerId,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

#[derive(Iden)]
enum Task {
    Table,
    Id,
    Title,
    Body,
    Status,
    Position,
    AssigneeId,
    DueAt,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

#[derive(Iden)]
enum Note {
    Table,
    Id,
    Title,
    BodyV2,
    Position,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

#[derive(Iden)]
enum TaskTarget {
    Table,
    Id,
    TaskId,
    PersonId,
    CompanyId,
    OpportunityId,
    CreatedAt,
}

#[derive(Iden)]
enum Workflow {
    Table,
    Id,
    Name,
    LastPublishedVersionId,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum WorkflowVersion {
    Table,
    Id,
    WorkflowId,
    Status,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum WorkflowVersionStep {
    Table,
    Id,
    WorkflowVersionId,
    Type,
    Settings,
    Position,
    CreatedAt,
}

#[derive(Iden)]
enum WorkflowRun {
    Table,
    Id,
    WorkflowVersionId,
    Status,
    Output,
    Error,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum ConnectedAccount {
    Table,
    Id,
    Provider,
    AccountOwnerId,
    Status,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum CalendarEvent {
    Table,
    Id,
    ConnectedAccountId,
    Title,
    StartTime,
    EndTime,
    Description,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum CalendarEventParticipant {
    Table,
    Id,
    CalendarEventId,
    Email,
    PersonId,
    CreatedAt,
}
