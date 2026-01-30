use crate::application::ports::storage::StorageProvider;
use async_trait::async_trait;
use std::path::PathBuf;
use tokio::fs;

#[derive(Debug)]
pub struct FileSystemStorage {
    root_dir: PathBuf,
}

impl FileSystemStorage {
    pub fn new(root_dir: PathBuf) -> Self {
        Self { root_dir }
    }
}

#[async_trait]
impl StorageProvider for FileSystemStorage {
    async fn upload(&self, file_name: &str, data: Vec<u8>) -> Result<String, String> {
        let path = self.root_dir.join(file_name);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .await
                .map_err(|e| e.to_string())?;
        }
        fs::write(&path, data).await.map_err(|e| e.to_string())?;
        Ok(path.to_string_lossy().into_owned())
    }

    async fn delete(&self, file_name: &str) -> Result<(), String> {
        let path = self.root_dir.join(file_name);
        if path.exists() {
            fs::remove_file(path).await.map_err(|e| e.to_string())?;
        }
        Ok(())
    }
}
