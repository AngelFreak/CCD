use crate::models::*;
use anyhow::{Context, Result};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

const DEFAULT_PB_URL: &str = "http://localhost:8090";

/// PocketBase API client
#[derive(Clone)]
pub struct PocketBaseClient {
    client: Client,
    base_url: String,
}

impl PocketBaseClient {
    /// Create a new PocketBase client
    pub fn new(base_url: Option<String>) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self {
            client,
            base_url: base_url.unwrap_or_else(|| DEFAULT_PB_URL.to_string()),
        })
    }

    /// Get the base URL for API requests
    fn api_url(&self, collection: &str) -> String {
        format!("{}/api/collections/{}/records", self.base_url, collection)
    }

    /// Generic GET request to fetch all records
    async fn list<T: for<'de> Deserialize<'de>>(
        &self,
        collection: &str,
        filter: Option<&str>,
        sort: Option<&str>,
    ) -> Result<Vec<T>> {
        let mut url = self.api_url(collection);
        let mut params = vec![];

        if let Some(f) = filter {
            params.push(format!("filter={}", urlencoding::encode(f)));
        }
        if let Some(s) = sort {
            params.push(format!("sort={}", urlencoding::encode(s)));
        }

        // PocketBase uses perPage for pagination, set to max
        params.push("perPage=500".to_string());

        if !params.is_empty() {
            url = format!("{}?{}", url, params.join("&"));
        }

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to send GET request")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("Request failed with status {}: {}", status, body);
        }

        #[derive(Deserialize)]
        struct ListResponse<T> {
            items: Vec<T>,
        }

        let list_response: ListResponse<T> = response
            .json()
            .await
            .context("Failed to parse response")?;

        Ok(list_response.items)
    }

    /// Generic GET request to fetch a single record
    async fn get<T: for<'de> Deserialize<'de>>(
        &self,
        collection: &str,
        id: &str,
    ) -> Result<T> {
        let url = format!("{}/{}", self.api_url(collection), id);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to send GET request")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("Request failed with status {}: {}", status, body);
        }

        response.json().await.context("Failed to parse response")
    }

    /// Generic POST request to create a record
    async fn create<T: for<'de> Deserialize<'de>, P: Serialize>(
        &self,
        collection: &str,
        payload: P,
    ) -> Result<T> {
        let url = self.api_url(collection);

        let response = self
            .client
            .post(&url)
            .json(&payload)
            .send()
            .await
            .context("Failed to send POST request")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("Request failed with status {}: {}", status, body);
        }

        response.json().await.context("Failed to parse response")
    }

    /// Generic PATCH request to update a record
    async fn update<T: for<'de> Deserialize<'de>, P: Serialize>(
        &self,
        collection: &str,
        id: &str,
        payload: P,
    ) -> Result<T> {
        let url = format!("{}/{}", self.api_url(collection), id);

        let response = self
            .client
            .patch(&url)
            .json(&payload)
            .send()
            .await
            .context("Failed to send PATCH request")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("Request failed with status {}: {}", status, body);
        }

        response.json().await.context("Failed to parse response")
    }

    /// Generic DELETE request to delete a record
    async fn delete(&self, collection: &str, id: &str) -> Result<()> {
        let url = format!("{}/{}", self.api_url(collection), id);

        let response = self
            .client
            .delete(&url)
            .send()
            .await
            .context("Failed to send DELETE request")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("Request failed with status {}: {}", status, body);
        }

        Ok(())
    }

    // ==================== PROJECT METHODS ====================

    /// List all projects
    pub async fn list_projects(&self, status_filter: Option<ProjectStatus>) -> Result<Vec<Project>> {
        let filter = status_filter.map(|s| format!("status='{}'", s.as_str()));
        self.list("projects", filter.as_deref(), Some("-updated")).await
    }

    /// Get a single project by ID
    pub async fn get_project(&self, id: &str) -> Result<Project> {
        self.get("projects", id).await
    }

    /// Create a new project
    pub async fn create_project(&self, payload: ProjectPayload) -> Result<Project> {
        self.create("projects", payload).await
    }

    /// Update a project
    pub async fn update_project(&self, id: &str, payload: ProjectPayload) -> Result<Project> {
        self.update("projects", id, payload).await
    }

    /// Delete a project
    pub async fn delete_project(&self, id: &str) -> Result<()> {
        self.delete("projects", id).await
    }

    // ==================== CONTEXT SECTION METHODS ====================

    /// List context sections for a project
    pub async fn list_context_sections(&self, project_id: &str) -> Result<Vec<ContextSection>> {
        let filter = format!("project='{}'", project_id);
        self.list("context_sections", Some(&filter), Some("order")).await
    }

    /// Get a single context section by ID
    pub async fn get_context_section(&self, id: &str) -> Result<ContextSection> {
        self.get("context_sections", id).await
    }

    /// Create a new context section
    pub async fn create_context_section(&self, payload: ContextSectionPayload) -> Result<ContextSection> {
        self.create("context_sections", payload).await
    }

    /// Update a context section
    pub async fn update_context_section(&self, id: &str, payload: ContextSectionPayload) -> Result<ContextSection> {
        self.update("context_sections", id, payload).await
    }

    /// Delete a context section
    pub async fn delete_context_section(&self, id: &str) -> Result<()> {
        self.delete("context_sections", id).await
    }

    // ==================== SESSION METHODS ====================

    /// List session history for a project
    pub async fn list_sessions(&self, project_id: &str) -> Result<Vec<SessionHistory>> {
        let filter = format!("project='{}'", project_id);
        self.list("session_history", Some(&filter), Some("-session_start")).await
    }

    /// Get a single session by ID
    pub async fn get_session(&self, id: &str) -> Result<SessionHistory> {
        self.get("session_history", id).await
    }

    /// Create a new session
    pub async fn create_session(&self, payload: SessionPayload) -> Result<SessionHistory> {
        self.create("session_history", payload).await
    }

    /// Update a session
    pub async fn update_session(&self, id: &str, payload: SessionPayload) -> Result<SessionHistory> {
        self.update("session_history", id, payload).await
    }

    /// Delete a session
    pub async fn delete_session(&self, id: &str) -> Result<()> {
        self.delete("session_history", id).await
    }

    // ==================== FACT METHODS ====================

    /// List extracted facts for a project
    pub async fn list_facts(&self, project_id: &str, include_stale: bool) -> Result<Vec<ExtractedFact>> {
        let mut filter = format!("project='{}'", project_id);
        if !include_stale {
            filter.push_str(" && stale=false");
        }
        self.list("extracted_facts", Some(&filter), Some("-importance,-created")).await
    }

    /// Get facts by type for a project
    pub async fn list_facts_by_type(&self, project_id: &str, fact_type: FactType) -> Result<Vec<ExtractedFact>> {
        let filter = format!("project='{}' && fact_type='{}'", project_id, fact_type.as_str());
        self.list("extracted_facts", Some(&filter), Some("-importance,-created")).await
    }

    /// Get a single fact by ID
    pub async fn get_fact(&self, id: &str) -> Result<ExtractedFact> {
        self.get("extracted_facts", id).await
    }

    /// Create a new fact
    pub async fn create_fact(&self, payload: ExtractedFactPayload) -> Result<ExtractedFact> {
        self.create("extracted_facts", payload).await
    }

    /// Update a fact
    pub async fn update_fact(&self, id: &str, payload: ExtractedFactPayload) -> Result<ExtractedFact> {
        self.update("extracted_facts", id, payload).await
    }

    /// Delete a fact
    pub async fn delete_fact(&self, id: &str) -> Result<()> {
        self.delete("extracted_facts", id).await
    }

    /// Mark a fact as stale
    pub async fn mark_fact_stale(&self, id: &str) -> Result<ExtractedFact> {
        #[derive(Serialize)]
        struct StalePayload {
            stale: bool,
        }

        self.update("extracted_facts", id, StalePayload { stale: true }).await
    }

    // ==================== UTILITY METHODS ====================

    /// Check if the PocketBase server is reachable
    pub async fn health_check(&self) -> bool {
        let url = format!("{}/api/health", self.base_url);
        self.client
            .get(&url)
            .send()
            .await
            .map(|r| r.status().is_success())
            .unwrap_or(false)
    }

    /// Get server information
    pub async fn get_server_info(&self) -> Result<serde_json::Value> {
        let url = format!("{}/api/health", self.base_url);
        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to get server info")?;

        response.json().await.context("Failed to parse server info")
    }
}

/// Shared PocketBase client instance
pub type SharedPocketBaseClient = Arc<PocketBaseClient>;

impl Default for PocketBaseClient {
    fn default() -> Self {
        Self::new(None).expect("Failed to create default PocketBase client")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_url() {
        let client = PocketBaseClient::new(None).unwrap();
        assert_eq!(
            client.api_url("projects"),
            "http://localhost:8090/api/collections/projects/records"
        );
    }

    #[test]
    fn test_custom_base_url() {
        let client = PocketBaseClient::new(Some("http://example.com:9000".to_string())).unwrap();
        assert_eq!(
            client.api_url("projects"),
            "http://example.com:9000/api/collections/projects/records"
        );
    }
}
