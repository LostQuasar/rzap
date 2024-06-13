use crate::data_type::*;
use reqwest::header;
use std::{error::Error, fmt::Debug};
use strum_macros::EnumString;

pub struct OpenShockAPI {
    client: reqwest::Client,
    base_url: String,
    default_key: String,
}

#[derive(EnumString, Debug)]
pub enum ShockerSource {
    Own,
    Shared,
}

impl OpenShockAPI {
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

    pub async fn get_shockers(
        &self,
        source: ShockerSource,
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

    pub async fn post_control(
        &self,
        id: String,
        control_type: ControlType,
        api_key: Option<String>,
    ) -> Result<String, Box<dyn Error>> {
        let control_request = serde_json::to_string(&ControlRequest {
            shocks: vec![Shock {
                id: id,
                control_type: control_type,
                intensity: 1,
                duration: 300,
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
