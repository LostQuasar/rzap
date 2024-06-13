use crate::data_type::*;
use reqwest::header;
use std::{error::Error, fmt::Debug};
use strum_macros::EnumString;

/// All methods contain an `Option<String>` to provide an alternate api key to use if it differs from the default
pub struct OpenShockAPI {
    client: reqwest::Client,
    base_url: String,
    default_key: String,
}

/// Which list of shockers to return
#[derive(EnumString, Debug)]
pub enum ListShockerSource {
    Own,
    Shared,
}

impl OpenShockAPI {
    /// Create a new instance of the api interface with a default key and the base_url, because OpenShock can be self hosted `base_url` can be any url without the leading `/` if `None` is provided the default of <https://api.shocklink.net> is used.
    pub fn new(base_url: Option<String>, default_key: String) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Content-type",
            header::HeaderValue::from_static("application/json"),
        );
        headers.insert(
            "accept",
            header::HeaderValue::from_static("application/json"),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        let base_url = base_url.unwrap_or("https://api.shocklink.net".to_string());
        OpenShockAPI {
            client,
            base_url,
            default_key,
        }
    }

    /// Gets user info from the provided API key, the default key from the instance is used if `None` is provided
    pub async fn get_user_info(
        &self,
        api_key: Option<String>,
    ) -> Result<SelfResponse, Box<dyn Error>> {
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
    ) -> Result<Vec<ListShockersResponse>, Box<dyn Error>> {
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
    ) -> Result<String, Box<dyn Error>> {
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
            custom_name: "rusty".to_string(),
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
