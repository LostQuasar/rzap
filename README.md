# rzap 

This library provides an interface to controll shocker devices via [OpenShock](http://openshock.org)'s API

NOTE: This is an un-official API iterface created by someone who has just started learning rust, no guarantees are made and contributions are greatly welcomed

```
[dependencies]
reqwest = { version = "0.11.27" }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
strum_macros = "0.26.4"
strum = "0.26.2"
tokio = { version = "1.21.2", features = ["macros", "rt-multi-thread"] }
```

## Example 

A simple request to retrieve the API key user's id

```rs
    dotenv().ok();
    let openshock_token = dotenv::var("OPENSHOCK_TOKEN").expect("missing OPENSHOCK_TOKEN");
    let app_name = env!("CARGO_PKG_NAME");
    let app_version = env!("CARGO_PKG_VERSION");

    assert_ne!(openshock_token, "");

    openshock_api = OpenShockAPIBuilder::new()
        .with_app(app_name.to_string(), Some(app_version.to_string()))
        .with_default_api_token(openshock_token)
        .build()
        .unwrap();
    
    println!(openshock_api.get_user_info(None).await.unwrap().id);
```

