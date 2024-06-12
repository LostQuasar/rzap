//! # rzap
//!
//! A rust library for connecting to an openshock server

pub mod data_type;

use data_type::data_type::*;
use reqwest::Client;
use std::{error::Error, fmt::Debug};
use strum_macros::EnumString;

/// Sends a request to the openshock server to control the shocker device
pub async fn post_control(
    client: &Client,
    api_url: &str,
    id: String,
    control_type: ControlType,
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

    let resp = client
        .post(format!("{api_url}/2/shockers/control"))
        .body(control_request)
        .send()
        .await?;
    let base_response: BaseResponse<String> = serde_json::from_str(resp.text().await?.as_str())?;
    Ok(base_response.message.unwrap())
}

///Gets a list of shockers including the device they are paired to
pub async fn get_shockers(
    client: &Client,
    api_url: &str,
    source: ShockerSource,
) -> Result<Vec<ListShockersResponse>, Box<dyn Error>> {
    let resp = client
        .get(format!("{}/1/shockers/{:?}", api_url, source))
        .send()
        .await;
    let list_shockers_response: BaseResponse<Vec<ListShockersResponse>> =
        serde_json::from_str(resp?.text().await?.as_str())?;
    //I dont like this
    Ok(list_shockers_response.data.unwrap())
}

pub async fn get_user_info(client: &Client, api_url: &str) -> Result<SelfResponse, Box<dyn Error>> {
    let resp = client.get(format!("{}/1/users/self", api_url)).send().await;
    let self_base_response: BaseResponse<SelfResponse> =
        serde_json::from_str(resp?.text().await?.as_str())?;
    //I dont like this
    Ok(self_base_response.data.unwrap())
}

#[derive(EnumString, Debug)]
pub enum ShockerSource {
    Own,
    Shared,
}

#[cfg(test)]
mod tests {
    use std::hash::{DefaultHasher, Hash, Hasher};

    use super::*;
    use dotenv::dotenv;
    use reqwest::header;

    fn setup() -> (Client, String) {
        dotenv().ok();
        let openshock_token = dotenv::var("OPENSHOCK_TOKEN").expect("missing OPENSHOCK_TOKEN");
        let api_url = "https://api.shocklink.net";

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
            "OpenShockToken",
            header::HeaderValue::from_str(&openshock_token).unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        (client, api_url.to_string())
    }

    fn calculate_hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }

    #[tokio::test]
    async fn get_shockers_test() {
        dotenv().ok();
        let shocker_test_id = dotenv::var("SHOCKER_TEST_ID").expect("missing SHOCKER_TEST_ID");

        let (client, api_url) = setup();
        let result = get_shockers(&client, api_url.as_str(), ShockerSource::Own);
        assert_eq!(
            calculate_hash(&result.await.unwrap()[0].shockers[0].id),
            calculate_hash(&shocker_test_id)
        );
    }

    #[tokio::test]
    async fn post_control_test() {
        dotenv().ok();
        let shocker_test_id = dotenv::var("SHOCKER_TEST_ID").expect("missing SHOCKER_TEST_ID");

        let (client, api_url) = setup();
        let result = post_control(
            &client,
            api_url.as_str(),
            shocker_test_id,
            ControlType::Sound,
        );
        assert_eq!(
            &result.await.unwrap(),
            &"Successfully sent control messages"
        );
    }

    #[tokio::test]
    async fn get_user_info_test() {
        dotenv().ok();
        let user_test_id = dotenv::var("USER_TEST_ID").expect("missing USER_TEST_ID");

        let (client, api_url) = setup();
        let result = get_user_info(&client, api_url.as_str());
        assert_eq!(
            calculate_hash(&result.await.unwrap().id),
            calculate_hash(&user_test_id)
        );
    }
}
