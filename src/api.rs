use crate::{api_builder::OpenShockAPIBuilder, data_type::*, error::Error};
use std::fmt::Debug;
use strum_macros::EnumString;

/// All methods contain an `Option<String>` to provide an alternate api key to use if it differs from the default 
/// Should they? probably not there gotta be a better way to do this.
pub struct OpenShockAPI {
    pub client: reqwest::Client,
    pub base_url: String,
    pub default_key: String,
    pub app_name: String,
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
