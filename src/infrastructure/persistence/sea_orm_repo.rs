use super::entities::opportunity::{self, Entity as OpportunityEntity};
use crate::application::ports::output::{
    OpportunityRepository, UserRepository, WorkspaceRepository,
};
use crate::domain::{
    DomainError, Opportunity, OpportunityStage, Person, User, Workspace, WorkspaceMember,
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
