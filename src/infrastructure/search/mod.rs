use crate::application::ports::search::SearchIndex;
use async_trait::async_trait;
use serde_json::Value;

#[derive(Debug)]
pub struct MockSearchIndex;

#[async_trait]
impl SearchIndex for MockSearchIndex {
    async fn index_document(
        &self,
        _index: &str,
        _id: &str,
        _document: Value,
    ) -> Result<(), String> {
        // No-op for mock
        println!("MockSearchIndex: Indexing document");
        Ok(())
    }

    async fn search(&self, _index: &str, _query: &str) -> Result<Vec<Value>, String> {
        // Return empty for mock
        println!("MockSearchIndex: Searching");
        Ok(vec![])
    }
}
