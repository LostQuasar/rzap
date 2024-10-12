use reqwest::header;
use crate::{api::OpenShockAPI, error::Error};

/// Builder for [`OpenShockAPI`]
#[derive(Default)]
pub struct OpenShockAPIBuilder {
    base_url: Option<String>,
    default_key: Option<String>,
    app_name: Option<String>,
    app_version: Option<String>,
}

impl OpenShockAPIBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// set the base URL to use
    ///
    /// this is optional and can be provided to use a self-hosted instance of the OpenShock API. if
    /// left unset, the default (`https://api.openshock.app`) will be used.
    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.base_url = Some(base_url);
        self
    }

    /// set the API token to use
    ///
    /// this must be provided
    pub fn with_default_api_token(mut self, default_api_token: String) -> Self {
        self.default_key = Some(default_api_token);
        self
    }

    /// set the name and optionally version of the app using this crate
    ///
    /// this is optional. if provided, the information will be added to the user agent string for
    /// all OpenShock API requests and also sent in [`OpenShockAPI::post_control`] so the app name
    /// shows up in the OpenShock log.
    pub fn with_app(mut self, app_name: String, app_version: Option<String>) -> Self {
        self.app_name = Some(app_name);
        self.app_version = app_version;
        self
    }

    /// check parameters and build an instance of [`OpenShockAPI`]
    pub fn build(self) -> Result<OpenShockAPI, Error> {
        let base_url = self
            .base_url
            .unwrap_or("https://api.openshock.app".to_string());
        let Some(default_key) = self.default_key else {
            return Err(Error::MissingApiToken);
        };

        let mut user_agent = format!("rzap/{}", env!("CARGO_PKG_VERSION"));
        // maybe add platform information as well?
        let app_name = if let Some(app_name) = self.app_name {
            if let Some(app_version) = self.app_version {
                user_agent += &format!(" ({} {})", app_name, app_version);
            } else {
                user_agent += &format!(" ({})", app_name);
            }
            app_name
        } else {
            "rzap".to_string()
        };

        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Content-type",
            header::HeaderValue::from_static("application/json"),
        );
        headers.insert(
            "accept",
            header::HeaderValue::from_static("application/json"),
        );
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_str(&user_agent).map_err(|e| Error::InvalidHeaderValue(e))?,
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Ok(OpenShockAPI {
            client,
            base_url,
            default_key,
            app_name,
        })
    }
}
