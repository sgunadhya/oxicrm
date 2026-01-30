use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait SearchIndex: Send + Sync {
    async fn index_document(&self, index: &str, id: &str, document: Value) -> Result<(), String>;
    async fn search(&self, index: &str, query: &str) -> Result<Vec<Value>, String>;
}
