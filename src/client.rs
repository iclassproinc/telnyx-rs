use reqwest::{Client};
use serde::{Serialize, de::DeserializeOwned};
use std::time::Duration;

use crate::error::TelnyxError;

pub struct TelnyxClient {
    pub(crate) http_client: Client,
    pub(crate) api_key: String,
    pub (crate) base_url: String
}

/// Builder for construction a [`TelnyxClient`]
#[derive(Default)]
pub struct TenlyxClientBuilder {
    api_key: Option<String>,
    base_url: Option<String>,
    timeout: Option<Duration>
}

impl TelnyxClient {
    pub fn builder() -> TenlyxClientBuilder {
        TenlyxClientBuilder::default()
    }

    pub(crate) async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, TelnyxError> {
        let url = format!("{}{}", self.base_url, path);
        let response = self.http_client.get(&url).bearer_auth(&self.api_key).send().await?;

        self.parse_response(response).await
    }

    pub(crate) async fn post<T, B> (&self, path: &str,body: &B) -> Result<T, TelnyxError> where T: DeserializeOwned, B: Serialize {
        let url = format!("{}{}", self.base_url, path);
        let response = self.http_client.post(&url).bearer_auth(&self.api_key).json(body).send().await?;

        self.parse_response(response).await
    }

    pub(crate) async fn put<T, B> (&self, path: &str,body: &B) -> Result<T, TelnyxError> where T: DeserializeOwned, B: Serialize {
        let url = format!("{}{}", self.base_url, path);
        let response = self.http_client.put(&url).bearer_auth(&self.api_key).json(body).send().await?;

        self.parse_response(response).await
    }

    pub(crate) async fn patch<T, B> (&self, path: &str,body: &B) -> Result<T, TelnyxError> where T: DeserializeOwned, B: Serialize {
        let url = format!("{}{}", self.base_url, path);
        let response = self.http_client.patch(&url).bearer_auth(&self.api_key).json(body).send().await?;

        self.parse_response(response).await
    }

    pub(crate) async fn delete(&self, path: &str) -> Result<(), TelnyxError> {
        let url = format!("{}{}", self.base_url, path);
        let response = self.http_client.delete(&url).bearer_auth(&self.api_key).send().await?;

        if response.status().is_success() {
            Ok(())
        }
        else {
            Err(TelnyxError::Api { 
                status: response.status().as_u16(), 
                message: response.text().await.unwrap_or_default()
            })
        }
    }

    async fn parse_response<T: DeserializeOwned>(&self, response: reqwest::Response) -> Result<T, TelnyxError> {
        if response.status().is_success() {
            let body = response.text().await?;
            serde_json::from_str(&body).map_err(TelnyxError::from)
        }
        else {
            Err(TelnyxError::Api { 
                status: response.status().as_u16(), 
                message: response.text().await.unwrap_or_default()
            })
        }
    }
}

impl TenlyxClientBuilder {
    /// Set the API key (required)
    pub fn api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into());
        self
    }

    /// Set the base URL (optional, defaults to Telnyx production API)
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Sets the request timeout (optional, defaults to 30 seconds)
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Build the client
    pub fn build(self) -> Result<TelnyxClient, TelnyxError> {
        let api_key = self.api_key.ok_or_else(|| TelnyxError::Config("API key is required".into()))?;
        let base_url = self.base_url.unwrap_or_else(|| "https://api.telnyx.com/v2".into());
        let timeout = self.timeout.unwrap_or(Duration::from_secs(30));

        let http_client = Client::builder().timeout(timeout).build().map_err(TelnyxError::Http)?;

        Ok(TelnyxClient{
            http_client,
            api_key,
            base_url
        })
    }
}