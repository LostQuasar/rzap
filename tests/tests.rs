use dotenv::dotenv;
use rzap::{api::{OpenShockAPI, ShockerSource}, data_type::ControlType};
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
    let result = openshock_api
        .post_control(shocker_test_id, ControlType::Sound, None)
        .await;
    assert_eq!(&result.unwrap(), &"Successfully sent control messages");
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
