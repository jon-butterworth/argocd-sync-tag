use crate::app::*;
use crate::error::Error;
use reqwest::Client;
use serde::Serialize;
use serde_json::Value;
use tracing::{debug, info};

#[derive(Serialize)]
pub struct SyncRequest {
    revision: String,
    prune: bool,
    dry_run: bool,
    strategy: Strategy,
}

#[derive(Serialize)]
pub struct Strategy {
    hook: Hook,
}

#[derive(Serialize)]
pub struct Hook {
    force: bool,
}

impl SyncRequest {
    pub fn new() -> Self {
        SyncRequest {
            revision: "HEAD".to_string(),
            prune: false,
            dry_run: false,
            strategy: Strategy {
                hook: Hook { force: false },
            },
        }
    }
}

pub struct BuildApi {
    client: Client,
    address: String,
    token: String,
    debug: bool,
}

impl BuildApi {
    pub fn new(address: String, token: String, debug: bool) -> Self {
        let client = Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .expect("Failed to build client");

        Self {
            client,
            address,
            token,
            debug,
        }
    }

    pub async fn get_application(&self, app_name: &str) -> Result<Application, Error> {
        let url = format!("{}/api/v1/applications/{}", self.address, app_name);
        let response = self
            .client
            .get(url)
            .header("Content-Type", "application/json")
            .bearer_auth(&self.token)
            .send()
            .await?;

        let response_text = response.text().await?;

        if self.debug {
            debug!("Raw JSON response: {}", response_text);
        }

        // Check for error in the response
        if let Ok(json_value) = serde_json::from_str::<Value>(&response_text) {
            if json_value.get("error").is_some() {
                // Print the raw JSON error response
                eprintln!("Error response: {}", response_text);
                return Err(Error::Other(response_text));
            }
        }

        match serde_json::from_str::<Application>(&response_text) {
            Ok(application) => {
                if self.debug {
                    debug!(
                    "Parsed Application JSON: {}",
                    serde_json::to_string_pretty(&application)?
                );
                }
                Ok(application)
            }
            Err(err) => {
                // Print the raw JSON response if there is an error
                eprintln!("Failed to parse JSON response: {}", response_text);
                eprintln!("Error: {}", err);
                Err(Error::Serde(err))
            }
        }
    }

    pub async fn update_application(
        &self,
        app_name: &str,
        application: &Application,
    ) -> Result<(), Error> {
        let url = format!("{}/api/v1/applications/{}", self.address, app_name);
        self.client
            .put(url)
            .header("Content-Type", "application/json")
            .bearer_auth(&self.token)
            .json(application)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    pub async fn sync(&self, app_name: &str) -> Result<(), Error> {
        let url = format!("{}/api/v1/applications/{}/sync", self.address, app_name);
        let sync_request = SyncRequest::new();
        self.client
            .post(url)
            .bearer_auth(&self.token)
            .json(&sync_request)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }
}
