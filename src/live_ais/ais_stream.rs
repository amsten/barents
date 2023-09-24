use crate::live_ais::respose_structs::GetAISLatestResponses;

use super::respose_structs::TokenResponse;
use chrono::prelude::*;
use reqwest::{self, Client, StatusCode};
use std::collections::HashMap;
use thiserror::Error;

use log::{debug, error, info};

static BASE_URL: &'static str = "https://live.ais.barentswatch.no";

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

    #[error("no token available")]
    NoToken,
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
    token_fetched_time: Option<DateTime<Utc>>,
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

        debug!("fetch_token method - Value of URL: {}", url);

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
                self.token_fetched_time = Some(Utc::now());

                info!(
                    "Successfully fetched token. Expiry time: {:?}.",
                    self.token_expires_in
                );

                Ok(())
            }
            status_code => Err(FetchTokenError::UnexpectedStatusCode(status_code)),
        }
    }

    async fn refresh_token(&mut self) -> Result<(), FetchTokenError> {
        if let Some(token_fetched_time) = self.token_fetched_time {
            let duration_since_fetch = Utc::now()
                .signed_duration_since(token_fetched_time)
                .num_seconds();
            debug!("{}", duration_since_fetch);

            if duration_since_fetch < 3300 {
                debug!("Token is still valid for a long enough time, no need to refresh it.");
                return Ok(());
            }
        }

        self.fetch_token().await?;
        Ok(())
    }

    pub async fn get_latest_ais(&mut self, since: DateTime<Utc>) -> Result<(), FetchTokenError> {
        let url = reqwest::Url::parse(&format!(
            "{}/v1/latest/ais?since={}",
            BASE_URL,
            since.format("%Y-%m-%dT%H:%M:%S").to_string()
        ))
        .map_err(FetchTokenError::InvalidUrl)?;
        debug!("Method get_latest_ais - Value of URL: {}", url);

        self.refresh_token().await?;
        let token = self.token.as_deref().ok_or(FetchTokenError::NoToken)?;

        let res = self
            .client
            .get(url)
            .bearer_auth(token)
            .send()
            .await
            .map_err(FetchTokenError::NetworkError)?;
        match res.status() {
            StatusCode::OK => {
                let latest_ais_response: GetAISLatestResponses = res
                    .json::<GetAISLatestResponses>()
                    .await
                    .map_err(FetchTokenError::DeserializationError)?;

                info!("Successfully fetched and deserialized GetAISLatestResponse. Number of messages recieved: {}", latest_ais_response.len());

                Ok(())
            }
            status_code => Err(FetchTokenError::UnexpectedStatusCode(status_code)),
        }
    }

    
}
