use dotenv::dotenv;
use rzap::{api::{ListShockerSource, OpenShockAPI},api_builder::OpenShockAPIBuilder, data_type::{ControlType, SelfResponse}, error::Error};
use std::hash::{DefaultHasher, Hash, Hasher};


fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn get_test_api() -> OpenShockAPI {
    dotenv().ok();
    let openshock_token = dotenv::var("OPENSHOCK_TOKEN").expect("missing OPENSHOCK_TOKEN");
    let app_name = env!("CARGO_PKG_NAME");
    let app_version = env!("CARGO_PKG_VERSION");

    assert_ne!(openshock_token, "");

    OpenShockAPIBuilder::new()
        .with_app(app_name.to_string(), Some(app_version.to_string()))
        .with_default_api_token(openshock_token)
        .build()
        .unwrap()
}

#[tokio::test]
async fn get_shockers_test() {
    let openshock_api = get_test_api();
    let shocker_test_id = dotenv::var("SHOCKER_TEST_ID").expect("missing SHOCKER_TEST_ID");
    assert_ne!(shocker_test_id, "");

    let result = openshock_api.get_shockers(ListShockerSource::Own, None).await;
    assert_eq!(
        calculate_hash(&result.unwrap().unwrap()[0].shockers[0].id),
        calculate_hash(&shocker_test_id)
    );
}

#[tokio::test]
async fn post_control_test() {
    let openshock_api = get_test_api();
    let shocker_test_id = dotenv::var("SHOCKER_TEST_ID").expect("missing SHOCKER_TEST_ID");
    assert_ne!(shocker_test_id, "");

    let result = openshock_api
        .post_control(shocker_test_id, ControlType::Sound, 1, 300, None)
        .await;
    assert_eq!(&result.unwrap().unwrap(), &"Successfully sent control messages");
}

#[tokio::test]
async fn get_user_info_test() {
    let openshock_api = get_test_api();
    let user_test_id = dotenv::var("USER_TEST_ID").expect("missing USER_TEST_ID");
    assert_ne!(user_test_id, "");

    let result: Result<Option<SelfResponse>, Error> = openshock_api.get_user_info(None).await;
    assert_eq!(
        calculate_hash(&result.unwrap().unwrap().id),
        calculate_hash(&user_test_id)
    );
}