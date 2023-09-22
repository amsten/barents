use super::respose_structs::TokenResponse;
use reqwest::{self, Client, StatusCode};
use std::collections::HashMap;
use thiserror::Error;

use log::{debug, error, info};

#[derive(Error, Debug)]
pub enum FetchTokenError {
    #[error("network error: {0}")]
    NetworkError(reqwest::Error),

    #[error("invalid url: {0}")]
    InvalidUrl(url::ParseError),

    #[error("unexpected status code: {0}")]
    UnexpectedStatusCode(StatusCode),

    #[error("deserialization error: {0}")]
    DeserializationError(reqwest::Error),
}

pub enum ScopeType {
    Ais,
    Api,
}

impl ScopeType {
    fn as_str(&self) -> &str {
        match self {
            ScopeType::Ais => "ais",
            ScopeType::Api => "api",
        }
    }
}

pub struct AisLiveAPI {
    grant_type: String,
    client_id: String,
    client_secret: String,
    scope: ScopeType,
    client: Client,
    token: Option<String>,
    token_expires_in: Option<i64>,
    token_fetched_time: Option<chrono::DateTime<chrono::Utc>>,
}

impl AisLiveAPI {
    pub fn new(
        grant_type: String,
        client_id: String,
        client_secret: String,
        scope: ScopeType,
    ) -> Self {
        let client = reqwest::Client::new();

        return AisLiveAPI {
            grant_type,
            client_id,
            client_secret,
            scope,
            client,
            token: None,
            token_expires_in: None,
            token_fetched_time: None,
        };
    }

    pub async fn fetch_token(&mut self) -> Result<(), FetchTokenError> {
        let scope = &String::from(self.scope.as_str());

        let mut form = HashMap::new();
        form.insert("grant_type", &self.grant_type);
        form.insert("client_id", &self.client_id);
        form.insert("client_secret", &self.client_secret);
        form.insert("scope", scope);

        let url = reqwest::Url::parse("https://id.barentswatch.no/connect/token")
            .map_err(FetchTokenError::InvalidUrl)?;

        debug!("fetch_token method. Value of URL: {}", url);

        let res = self
            .client
            .post(url)
            .form(&form)
            .send()
            .await
            .map_err(FetchTokenError::NetworkError)?;
        match res.status() {
            StatusCode::OK => {
                let token_response: TokenResponse = res
                    .json::<TokenResponse>()
                    .await
                    .map_err(FetchTokenError::DeserializationError)?;
                self.token = Some(String::from(token_response.access_token));
                self.token_expires_in = Some(token_response.expires_in);
                self.token_fetched_time = Some(chrono::Utc::now());

                info!(
                    "Successfully fetched token. Expiry time: {:?}.",
                    self.token_expires_in
                );

                Ok(())
            }
            status_code => Err(FetchTokenError::UnexpectedStatusCode(status_code)),
        }
    }
}
