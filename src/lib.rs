//! # rzap 
//! 
//! This library provides an interface to control shocker devices via [OpenShock](http://openshock.org)'s API
//! 
//! **NOTE:** This is an un-official API interface created by someone who has just started learning rust, no guarantees are made and contributions are greatly welcomed
//! 
//! ```toml
//! [dependencies]
//! reqwest = { version = "0.11.27" }
//! serde = { version = "1.0", features = ["derive"] }
//! serde_json = "1.0"
//! strum_macros = "0.26.4"
//! strum = "0.26.2"
//! tokio = { version = "1.21.2", features = ["macros", "rt-multi-thread"] }
//! ```
//! 
//! ## Example 
//! 
//! A simple request to retrieve the API key user's id
//! 
//! ```rs
//! dotenv().ok();
//! let user_test_id = dotenv::var("USER_TEST_ID").expect("missing USER_TEST_ID");
//! let openshock_token = dotenv::var("OPENSHOCK_TOKEN").expect("missing OPENSHOCK_TOKEN");
//! 
//! let openshock_api = OpenShockAPI::new(None, openshock_token);
//! println!(openshock_api.get_user_info(None).await.unwrap().id);
//! ```
//! 

pub mod api;
pub mod data_type;
pub mod error;
