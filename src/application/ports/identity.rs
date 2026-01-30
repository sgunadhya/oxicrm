use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait IdentityProvider: Send + Sync {
    async fn get_current_user_id(&self) -> Result<Option<Uuid>, String>;
    async fn has_permission(&self, user_id: Uuid, permission: &str) -> Result<bool, String>;
}
