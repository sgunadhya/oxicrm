use async_trait::async_trait;

#[async_trait]
pub trait StorageProvider: Send + Sync {
    async fn upload(&self, file_name: &str, data: Vec<u8>) -> Result<String, String>; // Returns URL/Path
    async fn delete(&self, file_name: &str) -> Result<(), String>;
}
