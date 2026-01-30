use async_trait::async_trait;

#[async_trait]
pub trait BillingProvider: Send + Sync {
    async fn create_customer(&self, email: &str) -> Result<String, String>; // Returns external customer ID
    async fn create_subscription(&self, customer_id: &str, plan_id: &str)
        -> Result<String, String>;
}
