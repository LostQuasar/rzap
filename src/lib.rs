//! # rzap
//!
//! A rust library for connecting to an openshock server

pub mod data_type;

use data_type::data_type::*;
use reqwest::header;
use std::{error::Error, fmt::Debug};
use strum_macros::EnumString;

pub struct OpenShockAPI {
    base_url: String,
    client: reqwest::Client,
    default_key: String,
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
            base_url,
            client,
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

#[derive(EnumString, Debug)]
pub enum ShockerSource {
    Own,
    Shared,
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use std::hash::{DefaultHasher, Hash, Hasher};

    fn calculate_hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }

    #[tokio::test]
    async fn get_shockers_test() {
        dotenv().ok();
        let openshock_token = dotenv::var("OPENSHOCK_TOKEN").expect("missing OPENSHOCK_TOKEN");
        let shocker_test_id = dotenv::var("SHOCKER_TEST_ID").expect("missing SHOCKER_TEST_ID");
        assert_ne!(openshock_token, "");
        assert_ne!(shocker_test_id, "");

        let openshock_api = OpenShockAPI::new(None, openshock_token);

        let result = openshock_api.get_shockers(ShockerSource::Own, None).await;
        assert_eq!(
            calculate_hash(&result.unwrap()[0].shockers[0].id),
            calculate_hash(&shocker_test_id)
        );
    }

    #[tokio::test]
    async fn post_control_test() {
        dotenv().ok();
        let openshock_token = dotenv::var("OPENSHOCK_TOKEN").expect("missing OPENSHOCK_TOKEN");
        let shocker_test_id = dotenv::var("SHOCKER_TEST_ID").expect("missing SHOCKER_TEST_ID");
        assert_ne!(openshock_token, "");
        assert_ne!(shocker_test_id, "");

        let openshock_api = OpenShockAPI::new(None, openshock_token);
        let result = openshock_api.post_control(shocker_test_id, ControlType::Sound, None).await;
        assert_eq!(
            &result.unwrap(),
            &"Successfully sent control messages"
        );
    }

    #[tokio::test]
    async fn get_user_info_test() {
        dotenv().ok();
        let user_test_id = dotenv::var("USER_TEST_ID").expect("missing USER_TEST_ID");
        let openshock_token = dotenv::var("OPENSHOCK_TOKEN").expect("missing OPENSHOCK_TOKEN");
        assert_ne!(user_test_id, "");
        assert_ne!(openshock_token, "");

        let openshock_api = OpenShockAPI::new(None, openshock_token);
        let result = openshock_api.get_user_info(None).await;
        assert_eq!(
            calculate_hash(&result.unwrap().id),
            calculate_hash(&user_test_id)
        );
    }
}
