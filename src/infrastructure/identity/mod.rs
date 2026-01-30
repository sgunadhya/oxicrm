use crate::application::ports::identity::IdentityProvider;
use async_trait::async_trait;
use uuid::Uuid;

pub struct MockIdentityProvider;

#[async_trait]
impl IdentityProvider for MockIdentityProvider {
    async fn get_current_user_id(&self) -> Result<Option<Uuid>, String> {
        // Mock returning a fixed UUID or None
        Ok(Some(Uuid::nil()))
    }

    async fn has_permission(&self, _user_id: Uuid, _permission: &str) -> Result<bool, String> {
        // Mock allowing everything
        Ok(true)
    }
}
