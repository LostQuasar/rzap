use crate::{data_type::*, error::Error};
use reqwest::header;
use std::fmt::Debug;
use strum_macros::EnumString;

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

/// All methods contain an `Option<String>` to provide an alternate api key to use if it differs from the default
pub struct OpenShockAPI {
    client: reqwest::Client,
    base_url: String,
    default_key: String,
    app_name: String,
}

/// Which list of shockers to return
#[derive(EnumString, Debug)]
pub enum ListShockerSource {
    Own,
    Shared,
}

impl OpenShockAPI {
    /// Return a builder for the api interface
    ///
    /// this is the same as [`OpenShockAPIBuilder::new`]
    pub fn builder() -> OpenShockAPIBuilder {
        OpenShockAPIBuilder::new()
    }

    /// Create a new instance of the api interface with a default key and the base_url, because OpenShock can be self hosted `base_url` can be any url without the leading `/` if `None` is provided the default of <https://api.shocklink.net> is used.
    pub fn new(base_url: Option<String>, default_key: String) -> Self {
        let mut builder = Self::builder().with_default_api_token(default_key);
        if let Some(base_url) = base_url {
            builder = builder.with_base_url(base_url);
        }
        builder.build().unwrap()
    }

    /// Gets user info from the provided API key, the default key from the instance is used if `None` is provided
    pub async fn get_user_info(&self, api_key: Option<String>) -> Result<SelfResponse, Error> {
        let resp = self
            .client
            .get(format!("{}/1/users/self", self.base_url))
            .header(
                "OpenShockToken",
                api_key.unwrap_or(self.default_key.clone()),
            )
            .send()
            .await?;
        let self_base_response: BaseResponse<SelfResponse> =
            serde_json::from_str(resp.text().await?.as_str())?;
        Ok(self_base_response.data.unwrap())
    }

    /// Gets a list of shockers that the user has access to from either their own shockers or ones shared with them
    pub async fn get_shockers(
        &self,
        source: ListShockerSource,
        api_key: Option<String>,
    ) -> Result<Vec<ListShockersResponse>, Error> {
        let resp = self
            .client
            .get(format!("{}/1/shockers/{:?}", self.base_url, source))
            .header(
                "OpenShockToken",
                api_key.unwrap_or(self.default_key.clone()),
            )
            .send()
            .await?;
        let list_shockers_response: BaseResponse<Vec<ListShockersResponse>> =
            serde_json::from_str(resp.text().await?.as_str())?;
        Ok(list_shockers_response.data.unwrap())
    }

    ///Sends a control request to the api and returns the response message which should be "Successfully sent control messages" exactly if it was successful
    pub async fn post_control(
        &self,
        id: String,
        control_type: ControlType,
        intensity: u8,
        duration: u16,
        api_key: Option<String>,
    ) -> Result<String, Error> {
        match intensity {
            1..=100 => {}
            _ => {
                panic!("Intensity is outside of bounds");
            }
        }

        match duration {
            300..=30000 => {}
            _ => {
                panic!("Duration is outside of bounds");
            }
        }

        let control_request = serde_json::to_string(&ControlRequest {
            shocks: vec![Shock {
                id: id,
                control_type: control_type,
                intensity: intensity,
                duration: duration,
                exclusive: true,
            }],
            custom_name: self.app_name.clone(),
        })?;

        let resp = self
            .client
            .post(format!("{}/2/shockers/control", self.base_url))
            .body(control_request)
            .header(
                "OpenShockToken",
                api_key.unwrap_or(self.default_key.clone()),
            )
            .send()
            .await?;
        let base_response: BaseResponse<String> =
            serde_json::from_str(resp.text().await?.as_str())?;
        Ok(base_response.message.unwrap())
    }
}
