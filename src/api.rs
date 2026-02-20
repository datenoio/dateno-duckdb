use crate::error::{DatenoError, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tokio::runtime::Runtime;
use url::Url;

/// Dateno API client
pub struct DatenoClient {
    client: reqwest::Client,
    base_url: Url,
    api_key: Option<String>,
    runtime: Arc<Runtime>,
}

/// Search request parameters
pub struct SearchRequest {
    pub query: String,
    pub filters: Vec<String>,
    pub limit: i64,
}

/// Dataset metadata from Dateno API
#[derive(Debug, Clone, Deserialize)]
pub struct Dataset {
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub format: String,
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub url: String,
    #[serde(default, deserialize_with = "deserialize_optional_datetime")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(default, deserialize_with = "deserialize_optional_datetime")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub formats: Vec<String>,
}

/// Search response from Dateno API
#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    pub total: usize,
    pub results: Vec<Dataset>,
}

fn deserialize_optional_datetime<'de, D>(deserializer: D) -> std::result::Result<Option<DateTime<Utc>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(s) => {
            DateTime::parse_from_rfc3339(&s)
                .map(|dt| Some(dt.with_timezone(&Utc)))
                .map_err(serde::de::Error::custom)
        }
        None => Ok(None),
    }
}

impl DatenoClient {
    /// Create a new Dateno client
    pub fn new(api_key: Option<String>) -> Result<Self> {
        let runtime = Runtime::new()
            .map_err(|e| DatenoError::Runtime(format!("Failed to create Tokio runtime: {}", e)))?;

        let client = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(60))
            .pool_max_idle_per_host(10)
            .user_agent("dateno-duckdb-ext/0.1.0")
            .build()
            .map_err(|e| DatenoError::Http(e))?;

        let base_url = Url::parse("https://api.dateno.io/search/0.2/query")
            .map_err(|e| DatenoError::Url(e))?;

        Ok(Self {
            client,
            base_url,
            api_key,
            runtime: Arc::new(runtime),
        })
    }

    /// Execute a search request synchronously
    pub fn search(&self, request: SearchRequest) -> Result<SearchResponse> {
        // Validate limit
        if request.limit < 1 || request.limit > 100 {
            return Err(DatenoError::InvalidLimit(request.limit));
        }

        let runtime = self.runtime.clone();
        let client = self.client.clone();
        let base_url = self.base_url.clone();
        let api_key = self.api_key.clone();

        runtime.block_on(async move {
            Self::search_async(client, base_url, api_key, request).await
        })
    }

    async fn search_async(
        client: reqwest::Client,
        mut base_url: Url,
        api_key: Option<String>,
        request: SearchRequest,
    ) -> Result<SearchResponse> {
        // Build query parameters
        {
            let mut pairs = base_url.query_pairs_mut();
            pairs.append_pair("q", &request.query);
            pairs.append_pair("limit", &request.limit.to_string());

            // API key is sent as Bearer token in Authorization header (not query parameter)

            // Add filters
            for filter in &request.filters {
                let (key, value) = parse_filter(filter)?;
                let encoded = format!("\"{}\"=\"{}\"", key, value);
                pairs.append_pair("filters", &encoded);
            }
        }

        // Build request
        let mut http_request = client.get(base_url);

        // Add Bearer token if API key is provided (prefer header over query param)
        if let Some(key) = &api_key {
            http_request = http_request.bearer_auth(key);
        }

        // Execute request
        let response = http_request.send().await.map_err(|e| DatenoError::Http(e))?;

        // Check status
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(DatenoError::Api(format!(
                "API returned status {}: {}",
                status, body
            )));
        }

        // Parse JSON response
        let search_response: SearchResponse = response
            .json()
            .await
            .map_err(|e| DatenoError::Json(e))?;

        Ok(search_response)
    }
}

/// Parse a filter string into (key, value) tuple
fn parse_filter(filter: &str) -> Result<(String, String)> {
    let parts: Vec<&str> = filter.splitn(2, '=').collect();
    if parts.len() != 2 {
        return Err(DatenoError::InvalidFilter(format!(
            "Filter must be in format 'key=value', got: {}",
            filter
        )));
    }
    Ok((parts[0].to_string(), parts[1].to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_filter() {
        let (key, value) = parse_filter("source.countries.name=Canada").unwrap();
        assert_eq!(key, "source.countries.name");
        assert_eq!(value, "Canada");
    }

    #[test]
    fn test_parse_filter_invalid() {
        let result = parse_filter("invalid");
        assert!(result.is_err());
    }
}
