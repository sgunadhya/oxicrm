use super::entities::opportunity::{self, Entity as OpportunityEntity};
use crate::application::ports::output::{
    CalendarEventRepository, NoteRepository, OpportunityRepository, TaskRepository, TaskTargetRepository, TimelineActivityRepository, UserRepository, WorkflowRepository, WorkspaceRepository,
};
use crate::domain::{
    CalendarEvent, DomainError, Note, Opportunity, OpportunityStage, Person, Task, TaskTarget, TimelineActivity, User, Workflow, Workspace, WorkspaceMember,
};
use crate::infrastructure::persistence::entities::{person, user, workspace, workspace_member};
use async_trait::async_trait;
use sea_orm::*;
use uuid::Uuid;

pub struct SeaOrmRepo {
    pub db: DatabaseConnection,
}

#[async_trait]
impl crate::application::ports::output::PersonRepository for SeaOrmRepo {
    async fn find_by_email(&self, email: &str) -> Result<Option<Person>, DomainError> {
        let model = person::Entity::find()
            .filter(person::Column::Email.eq(email))
            .filter(person::Column::DeletedAt.is_null())
            .one(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
        Ok(model.map(|m| m.to_domain()))
    }

    async fn create(&self, person: Person) -> Result<Person, DomainError> {
        let model = person::ActiveModel {
            id: Set(person.id),
            name: Set(person.name),
            email: Set(person.email),
            position: Set(person.position),
            company_id: Set(person.company_id),
            created_at: Set(person.created_at.into()),
            updated_at: Set(person.updated_at.into()),
            deleted_at: Set(None),
        };

        let result = model
            .insert(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
        Ok(result.to_domain())
    }

    async fn find_all(&self) -> Result<Vec<Person>, DomainError> {
        let models = person::Entity::find()
            .filter(person::Column::DeletedAt.is_null())
            .all(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
        Ok(models.into_iter().map(|m| m.to_domain()).collect())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        // Soft delete
        let model = person::ActiveModel {
            id: Set(id),
            deleted_at: Set(Some(chrono::Utc::now().into())),
            ..Default::default()
        };
        model
            .update(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
        Ok(())
    }
}

#[async_trait]
impl OpportunityRepository for SeaOrmRepo {
    async fn find_all(&self) -> Result<Vec<Opportunity>, DomainError> {
        let models = OpportunityEntity::find()
            .filter(opportunity::Column::DeletedAt.is_null())
            .all(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
        Ok(models.into_iter().map(|m| m.to_domain()).collect())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Opportunity>, DomainError> {
        let model = OpportunityEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
        Ok(model.map(|m| m.to_domain()))
    }

    async fn create(&self, opportunity: Opportunity) -> Result<Opportunity, DomainError> {
        let stage_str = match opportunity.stage {
            OpportunityStage::Prospecting => "Prospecting",
            OpportunityStage::Qualification => "Qualification",
            OpportunityStage::Negotiation => "Negotiation",
            OpportunityStage::Won => "Won",
            OpportunityStage::Lost => "Lost",
        };

        let model = opportunity::ActiveModel {
            id: Set(opportunity.id),
            created_at: Set(opportunity.created_at.into()),
            updated_at: Set(opportunity.updated_at.into()),
            deleted_at: Set(None),
            name: Set(opportunity.name),
            stage: Set(stage_str.to_string()),
            amount_micros: Set(opportunity.amount_micros),
            currency_code: Set(opportunity.currency_code),
            close_date: Set(opportunity.close_date.map(|d| d.into())),
            company_id: Set(opportunity.company_id),
            point_of_contact_id: Set(opportunity.point_of_contact_id),
            owner_id: Set(opportunity.owner_id),
        };

        let result = model
            .insert(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
        Ok(result.to_domain())
    }

    async fn update(&self, opportunity: Opportunity) -> Result<Opportunity, DomainError> {
        let stage_str = match opportunity.stage {
            OpportunityStage::Prospecting => "Prospecting",
            OpportunityStage::Qualification => "Qualification",
            OpportunityStage::Negotiation => "Negotiation",
            OpportunityStage::Won => "Won",
            OpportunityStage::Lost => "Lost",
        };

        let model = opportunity::ActiveModel {
            id: Set(opportunity.id),
            updated_at: Set(chrono::Utc::now().into()),
            name: Set(opportunity.name),
            stage: Set(stage_str.to_string()),
            amount_micros: Set(opportunity.amount_micros),
            currency_code: Set(opportunity.currency_code),
            close_date: Set(opportunity.close_date.map(|d| d.into())),
            company_id: Set(opportunity.company_id),
            point_of_contact_id: Set(opportunity.point_of_contact_id),
            owner_id: Set(opportunity.owner_id),
            ..Default::default()
        };

        let result = model
            .update(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
        Ok(result.to_domain())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        let model = opportunity::ActiveModel {
            id: Set(id),
            deleted_at: Set(Some(chrono::Utc::now().into())),
            ..Default::default()
        };
        model
            .update(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
        Ok(())
    }
}

#[async_trait]
impl UserRepository for SeaOrmRepo {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError> {
        let model = user::Entity::find()
            .filter(user::Column::Email.eq(email))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Validation(e.to_string()))?;
        Ok(model.map(|m| m.to_domain()))
    }

    async fn create(&self, user: User) -> Result<User, DomainError> {
        let state_str = serde_json::to_value(user.state)
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();

        let active_model = user::ActiveModel {
            id: Set(user.id),
            email: Set(user.email.clone()),
            password_hash: Set(user.password_hash.clone()),
            state: Set(state_str),
            created_at: Set(user.created_at.into()),
            updated_at: Set(user.updated_at.into()),
        };

        let result = active_model
            .insert(&self.db)
            .await
            .map_err(|e| DomainError::Validation(e.to_string()))?;
        Ok(result.to_domain())
    }
}

#[async_trait]
impl WorkspaceRepository for SeaOrmRepo {
    async fn create(&self, workspace: Workspace) -> Result<Workspace, DomainError> {
        let state_str = serde_json::to_value(workspace.state)
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();

        let model = workspace::ActiveModel {
            id: Set(workspace.id),
            subdomain: Set(workspace.subdomain),
            state: Set(state_str),
            created_at: Set(workspace.created_at.into()),
            updated_at: Set(workspace.updated_at.into()),
        };

        let result = model
            .insert(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;

        Ok(result.to_domain())
    }

    async fn find_by_subdomain(&self, subdomain: &str) -> Result<Option<Workspace>, DomainError> {
        let result = workspace::Entity::find()
            .filter(workspace::Column::Subdomain.eq(subdomain))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;

        Ok(result.map(|m| m.to_domain()))
    }

    async fn add_member(&self, member: WorkspaceMember) -> Result<WorkspaceMember, DomainError> {
        let model = workspace_member::ActiveModel {
            id: Set(member.id),
            user_id: Set(member.user_id),
            workspace_id: Set(member.workspace_id),
            role: Set(member.role),
            name: Set(member.name),
            created_at: Set(member.created_at.into()),
            updated_at: Set(member.updated_at.into()),
        };

        let result = model
            .insert(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;

        Ok(result.to_domain())
    }
}

#[async_trait]
impl crate::application::ports::output::CompanyRepository for SeaOrmRepo {
    async fn find_all(&self) -> Result<Vec<crate::domain::Company>, DomainError> {
        use crate::infrastructure::persistence::entities::company;
        let models = company::Entity::find()
            .filter(company::Column::DeletedAt.is_null())
            .all(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
        Ok(models.into_iter().map(|m| m.to_domain()).collect())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<crate::domain::Company>, DomainError> {
        use crate::infrastructure::persistence::entities::company;
        let model = company::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
        Ok(model.map(|m| m.to_domain()))
    }

    async fn create(
        &self,
        company: crate::domain::Company,
    ) -> Result<crate::domain::Company, DomainError> {
        use crate::infrastructure::persistence::entities::company;
        let model = company::ActiveModel {
            id: Set(company.id),
            created_at: Set(company.created_at.into()),
            updated_at: Set(company.updated_at.into()),
            deleted_at: Set(None),
            name: Set(company.name),
            domain_name: Set(company.domain_name),
            address: Set(company.address),
            employees_count: Set(company.employees_count),
        };

        let result = model
            .insert(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
        Ok(result.to_domain())
    }

    async fn update(
        &self,
        company: crate::domain::Company,
    ) -> Result<crate::domain::Company, DomainError> {
        use crate::infrastructure::persistence::entities::company;
        let model = company::ActiveModel {
            id: Set(company.id),
            updated_at: Set(chrono::Utc::now().into()), // refresh updated_at
            name: Set(company.name),
            domain_name: Set(company.domain_name),
            address: Set(company.address),
            employees_count: Set(company.employees_count),
            ..Default::default()
        };

        let result = model
            .update(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
        Ok(result.to_domain())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        use crate::infrastructure::persistence::entities::company;
        let model = company::ActiveModel {
            id: Set(id),
            deleted_at: Set(Some(chrono::Utc::now().into())),
            ..Default::default()
        };
        model
            .update(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
        Ok(())
    }
}
#[async_trait]
impl crate::application::ports::output::TaskRepository for SeaOrmRepo {
    async fn find_all(&self) -> Result<Vec<crate::domain::Task>, DomainError> {
        use crate::infrastructure::persistence::entities::task;
        let models = task::Entity::find()
            .filter(task::Column::DeletedAt.is_null())
            .all(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
        Ok(models.into_iter().map(|m| m.to_domain()).collect())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<crate::domain::Task>, DomainError> {
        use crate::infrastructure::persistence::entities::task;
        let model = task::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
        Ok(model.map(|m| m.to_domain()))
    }

    async fn create(&self, task: crate::domain::Task) -> Result<crate::domain::Task, DomainError> {
        use crate::infrastructure::persistence::entities::task;
        let status_str = match task.status {
            crate::domain::states::TaskStatus::Todo => "TODO",
            crate::domain::states::TaskStatus::InProgress => "IN_PROGRESS",
            crate::domain::states::TaskStatus::Done => "DONE",
        };

        let model = task::ActiveModel {
            id: Set(task.id),
            created_at: Set(task.created_at.into()),
            updated_at: Set(task.updated_at.into()),
            deleted_at: Set(None),
            title: Set(task.title),
            body: Set(task.body),
            status: Set(status_str.to_string()),
            position: Set(task.position),
            assignee_id: Set(task.assignee_id),
            due_at: Set(task.due_at.map(|d| d.into())),
        };

        let result = model
            .insert(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
        Ok(result.to_domain())
    }

    async fn update(&self, task: crate::domain::Task) -> Result<crate::domain::Task, DomainError> {
        use crate::infrastructure::persistence::entities::task;
        let status_str = match task.status {
            crate::domain::states::TaskStatus::Todo => "TODO",
            crate::domain::states::TaskStatus::InProgress => "IN_PROGRESS",
            crate::domain::states::TaskStatus::Done => "DONE",
        };

        let model = task::ActiveModel {
            id: Set(task.id),
            updated_at: Set(chrono::Utc::now().into()),
            title: Set(task.title),
            body: Set(task.body),
            status: Set(status_str.to_string()),
            position: Set(task.position),
            assignee_id: Set(task.assignee_id),
            due_at: Set(task.due_at.map(|d| d.into())),
            ..Default::default()
        };

        let result = model
            .update(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
        Ok(result.to_domain())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        use crate::infrastructure::persistence::entities::task;
        let model = task::ActiveModel {
            id: Set(id),
            deleted_at: Set(Some(chrono::Utc::now().into())),
            ..Default::default()
        };
        model
            .update(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
        Ok(())
    }
}

#[async_trait]
impl crate::application::ports::output::NoteRepository for SeaOrmRepo {
    async fn find_all(&self) -> Result<Vec<crate::domain::Note>, DomainError> {
        use crate::infrastructure::persistence::entities::note;
        let models = note::Entity::find()
            .filter(note::Column::DeletedAt.is_null())
            .all(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
        Ok(models.into_iter().map(|m| m.to_domain()).collect())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<crate::domain::Note>, DomainError> {
        use crate::infrastructure::persistence::entities::note;
        let model = note::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
        Ok(model.map(|m| m.to_domain()))
    }

    async fn create(&self, note: crate::domain::Note) -> Result<crate::domain::Note, DomainError> {
        use crate::infrastructure::persistence::entities::note;
        let model = note::ActiveModel {
            id: Set(note.id),
            created_at: Set(note.created_at.into()),
            updated_at: Set(note.updated_at.into()),
            deleted_at: Set(None),
            title: Set(note.title),
            body_v2: Set(note.body_v2),
            position: Set(note.position),
        };

        let result = model
            .insert(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
        Ok(result.to_domain())
    }

    async fn update(&self, note: crate::domain::Note) -> Result<crate::domain::Note, DomainError> {
        use crate::infrastructure::persistence::entities::note;
        let model = note::ActiveModel {
            id: Set(note.id),
            updated_at: Set(chrono::Utc::now().into()),
            title: Set(note.title),
            body_v2: Set(note.body_v2),
            position: Set(note.position),
            ..Default::default()
        };

        let result = model
            .update(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
        Ok(result.to_domain())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        use crate::infrastructure::persistence::entities::note;
        let model = note::ActiveModel {
            id: Set(id),
            deleted_at: Set(Some(chrono::Utc::now().into())),
            ..Default::default()
        };
        model
            .update(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
        Ok(())
    }
}

#[async_trait]
impl crate::application::ports::output::TaskTargetRepository for SeaOrmRepo {
    async fn find_by_task_id(&self, task_id: Uuid) -> Result<Vec<crate::domain::TaskTarget>, DomainError> {
        use crate::infrastructure::persistence::entities::task_target;
        let models = task_target::Entity::find()
            .filter(task_target::Column::TaskId.eq(task_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
        Ok(models.into_iter().map(|m| m.to_domain()).collect())
    }

    async fn create(&self, task_target: crate::domain::TaskTarget) -> Result<crate::domain::TaskTarget, DomainError> {
        use crate::infrastructure::persistence::entities::task_target;
        let model = task_target::ActiveModel {
            id: Set(task_target.id),
            created_at: Set(task_target.created_at.into()),
            task_id: Set(task_target.task_id),
            person_id: Set(task_target.person_id),
            company_id: Set(task_target.company_id),
            opportunity_id: Set(task_target.opportunity_id),
        };

        let result = model
            .insert(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
        Ok(result.to_domain())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        use crate::infrastructure::persistence::entities::task_target;
        task_target::Entity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
        Ok(())
    }
}

#[async_trait]
impl WorkflowRepository for SeaOrmRepo {
    async fn find_all(&self) -> Result<Vec<Workflow>, DomainError> {
        use crate::infrastructure::persistence::entities::workflow;
        let models = workflow::Entity::find()
            .all(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
        Ok(models.into_iter().map(|m| m.to_domain()).collect())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Workflow>, DomainError> {
        use crate::infrastructure::persistence::entities::workflow;
        let model = workflow::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
        Ok(model.map(|m| m.to_domain()))
    }

    async fn create(&self, workflow: Workflow) -> Result<Workflow, DomainError> {
        use crate::infrastructure::persistence::entities::workflow;
        let model = workflow::ActiveModel {
            id: Set(workflow.id),
            name: Set(workflow.name),
            last_published_version_id: Set(workflow.last_published_version_id),
            created_at: Set(workflow.created_at.into()),
            updated_at: Set(workflow.updated_at.into()),
        };

        let result = model
            .insert(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
        Ok(result.to_domain())
    }

    async fn update(&self, workflow: Workflow) -> Result<Workflow, DomainError> {
        use crate::infrastructure::persistence::entities::workflow;
        let model = workflow::ActiveModel {
            id: Set(workflow.id),
            name: Set(workflow.name),
            last_published_version_id: Set(workflow.last_published_version_id),
            updated_at: Set(workflow.updated_at.into()),
            ..Default::default()
        };

        let result = model
            .update(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
        Ok(result.to_domain())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        use crate::infrastructure::persistence::entities::workflow;
        workflow::Entity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
        Ok(())
    }
}

#[async_trait]
impl CalendarEventRepository for SeaOrmRepo {
    async fn find_all(&self) -> Result<Vec<CalendarEvent>, DomainError> {
        use crate::infrastructure::persistence::entities::calendar_event;
        let models = calendar_event::Entity::find()
            .all(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
        Ok(models.into_iter().map(|m| m.to_domain()).collect())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<CalendarEvent>, DomainError> {
        use crate::infrastructure::persistence::entities::calendar_event;
        let model = calendar_event::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
        Ok(model.map(|m| m.to_domain()))
    }

    async fn create(&self, event: CalendarEvent) -> Result<CalendarEvent, DomainError> {
        use crate::infrastructure::persistence::entities::calendar_event;
        let model = calendar_event::ActiveModel {
            id: Set(event.id),
            connected_account_id: Set(event.connected_account_id),
            title: Set(event.title),
            start_time: Set(event.start_time.into()),
            end_time: Set(event.end_time.into()),
            description: Set(event.description),
            created_at: Set(event.created_at.into()),
            updated_at: Set(event.updated_at.into()),
        };

        let result = model
            .insert(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
        Ok(result.to_domain())
    }

    async fn update(&self, event: CalendarEvent) -> Result<CalendarEvent, DomainError> {
        use crate::infrastructure::persistence::entities::calendar_event;
        let model = calendar_event::ActiveModel {
            id: Set(event.id),
            connected_account_id: Set(event.connected_account_id),
            title: Set(event.title),
            start_time: Set(event.start_time.into()),
            end_time: Set(event.end_time.into()),
            description: Set(event.description),
            updated_at: Set(event.updated_at.into()),
            ..Default::default()
        };

        let result = model
            .update(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
        Ok(result.to_domain())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        use crate::infrastructure::persistence::entities::calendar_event;
        calendar_event::Entity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
        Ok(())
    }
}

#[async_trait]
impl TimelineActivityRepository for SeaOrmRepo {
    async fn find_all(&self) -> Result<Vec<TimelineActivity>, DomainError> {
        use crate::infrastructure::persistence::entities::timeline_activity;
        let models = timeline_activity::Entity::find()
            .order_by_desc(timeline_activity::Column::CreatedAt)
            .all(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
        Ok(models.into_iter().map(|m| m.to_domain()).collect())
    }

    async fn find_by_person_id(&self, person_id: Uuid) -> Result<Vec<TimelineActivity>, DomainError> {
        use crate::infrastructure::persistence::entities::timeline_activity;
        let models = timeline_activity::Entity::find()
            .filter(timeline_activity::Column::PersonId.eq(person_id))
            .order_by_desc(timeline_activity::Column::CreatedAt)
            .all(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
        Ok(models.into_iter().map(|m| m.to_domain()).collect())
    }

    async fn find_by_company_id(&self, company_id: Uuid) -> Result<Vec<TimelineActivity>, DomainError> {
        use crate::infrastructure::persistence::entities::timeline_activity;
        let models = timeline_activity::Entity::find()
            .filter(timeline_activity::Column::CompanyId.eq(company_id))
            .order_by_desc(timeline_activity::Column::CreatedAt)
            .all(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
        Ok(models.into_iter().map(|m| m.to_domain()).collect())
    }

    async fn find_by_opportunity_id(&self, opportunity_id: Uuid) -> Result<Vec<TimelineActivity>, DomainError> {
        use crate::infrastructure::persistence::entities::timeline_activity;
        let models = timeline_activity::Entity::find()
            .filter(timeline_activity::Column::OpportunityId.eq(opportunity_id))
            .order_by_desc(timeline_activity::Column::CreatedAt)
            .all(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
        Ok(models.into_iter().map(|m| m.to_domain()).collect())
    }

    async fn find_by_task_id(&self, task_id: Uuid) -> Result<Vec<TimelineActivity>, DomainError> {
        use crate::infrastructure::persistence::entities::timeline_activity;
        let models = timeline_activity::Entity::find()
            .filter(timeline_activity::Column::TaskId.eq(task_id))
            .order_by_desc(timeline_activity::Column::CreatedAt)
            .all(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
        Ok(models.into_iter().map(|m| m.to_domain()).collect())
    }

    async fn create(&self, activity: TimelineActivity) -> Result<TimelineActivity, DomainError> {
        use crate::infrastructure::persistence::entities::timeline_activity;
        let model = timeline_activity::ActiveModel {
            id: Set(activity.id),
            created_at: Set(activity.created_at.into()),
            name: Set(activity.name),
            workspace_member_id: Set(activity.workspace_member_id),
            person_id: Set(activity.person_id),
            company_id: Set(activity.company_id),
            opportunity_id: Set(activity.opportunity_id),
            task_id: Set(activity.task_id),
            note_id: Set(activity.note_id),
            calendar_event_id: Set(activity.calendar_event_id),
            workflow_id: Set(activity.workflow_id),
        };

        let result = model
            .insert(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
        Ok(result.to_domain())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        use crate::infrastructure::persistence::entities::timeline_activity;
        timeline_activity::Entity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
        Ok(())
    }
}
