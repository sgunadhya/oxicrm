use crate::application::ports::scheduling::{Job, JobQueue};
use async_trait::async_trait;
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct InMemoryJobQueue {
    sender: mpsc::Sender<Job>,
}

impl InMemoryJobQueue {
    pub fn new(sender: mpsc::Sender<Job>) -> Self {
        Self { sender }
    }
}

#[async_trait]
impl JobQueue for InMemoryJobQueue {
    async fn enqueue(&self, job: Job) -> Result<(), String> {
        self.sender.send(job).await.map_err(|e| e.to_string())?;
        Ok(())
    }
}
