use crate::application::ports::billing::BillingProvider;
use async_trait::async_trait;

#[derive(Debug)]
pub struct MockBillingProvider;

#[async_trait]
impl BillingProvider for MockBillingProvider {
    async fn create_customer(&self, email: &str) -> Result<String, String> {
        println!("MockBillingProvider: Creating customer for {}", email);
        Ok("cus_mock_123".to_string())
    }

    async fn create_subscription(
        &self,
        customer_id: &str,
        plan_id: &str,
    ) -> Result<String, String> {
        println!(
            "MockBillingProvider: Subscribing {} to {}",
            customer_id, plan_id
        );
        Ok("sub_mock_456".to_string())
    }
}
