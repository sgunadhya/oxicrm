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
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Opportunity>, DomainError> {
        let model = OpportunityEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Validation(e.to_string()))?;

        match model {
            Some(m) => {
                let stage = match m.stage.as_str() {
                    "New" => OpportunityStage::New,
                    "Meeting" => OpportunityStage::Meeting,
                    "Proposal" => OpportunityStage::Proposal,
                    "Customer" => OpportunityStage::Customer,
                    "Lost" => OpportunityStage::Lost,
                    _ => OpportunityStage::New,
                };

                Ok(Some(Opportunity {
                    id: m.id,
                    name: m.name,
                    stage,
                    amount_micros: m.amount_micros,
                    created_at: m.created_at,
                    updated_at: m.updated_at,
                    deleted_at: None,
                    position: 0,
                    close_date: None,
                    point_of_contact_id: None,
                    company_id: None,
                }))
            }
            None => Ok(None),
        }
    }

    async fn find_all(&self) -> Result<Vec<Opportunity>, DomainError> {
        let models = OpportunityEntity::find()
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Validation(e.to_string()))?;

        let opportunities = models
            .into_iter()
            .map(|m| {
                let stage = match m.stage.as_str() {
                    "New" => OpportunityStage::New,
                    "Meeting" => OpportunityStage::Meeting,
                    "Proposal" => OpportunityStage::Proposal,
                    "Customer" => OpportunityStage::Customer,
                    "Lost" => OpportunityStage::Lost,
                    _ => OpportunityStage::New,
                };
                Opportunity {
                    id: m.id,
                    name: m.name,
                    stage,
                    amount_micros: m.amount_micros,
                    created_at: m.created_at,
                    updated_at: m.updated_at,
                    deleted_at: None,
                    position: 0,
                    close_date: None,
                    point_of_contact_id: None,
                    company_id: None,
                }
            })
            .collect();

        Ok(opportunities)
    }

    async fn save(&self, opportunity: &Opportunity) -> Result<(), DomainError> {
        let stage_str = match opportunity.stage {
            OpportunityStage::New => "New",
            OpportunityStage::Meeting => "Meeting",
            OpportunityStage::Proposal => "Proposal",
            OpportunityStage::Customer => "Customer",
            OpportunityStage::Lost => "Lost",
        };

        let active_model = opportunity::ActiveModel {
            id: Set(opportunity.id),
            name: Set(opportunity.name.clone()),
            stage: Set(stage_str.to_string()),
            amount_micros: Set(opportunity.amount_micros),
            created_at: Set(opportunity.created_at),
            updated_at: Set(opportunity.updated_at),
        };

        let exists = OpportunityEntity::find_by_id(opportunity.id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Validation(e.to_string()))?;

        if exists.is_some() {
            active_model
                .update(&self.db)
                .await
                .map_err(|e| DomainError::Validation(e.to_string()))?;
        } else {
            active_model
                .insert(&self.db)
                .await
                .map_err(|e| DomainError::Validation(e.to_string()))?;
        }

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
