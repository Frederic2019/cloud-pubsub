use cloud_pubsub::Client;
use serde_derive::Deserialize;
use std::sync::Arc;
use   serde_json::json;
use std::collections::HashMap;

#[derive(Deserialize)]
struct Config {
    topic: String,
    google_application_credentials: String,
}

#[tokio::main]
async fn main() {
    let parsed_env = envy::from_env::<Config>();
    if let Err(e) = parsed_env {
        eprintln!("ENV is not valid: {}", e);
        std::process::exit(1);
    }
    let config = parsed_env.unwrap();

    let pubsub = match Client::new(config.google_application_credentials).await {
        Err(e) => panic!("Failed to initialize pubsub: {}", e),
        Ok(p) => Arc::new(p),
    };

    let mut attributes : HashMap<String,String>=HashMap::new();
    attributes.insert("Test_key".to_string(),"Test_Value".to_string());

    let topic = Arc::new(pubsub.topic(config.topic.clone()));
    match topic.clone().publish("🔥",attributes).await {
        Ok(response) => {
            println!("{:?}", response);
            pubsub.stop();
            std::process::exit(0);
        }
        Err(e) => eprintln!("Failed sending message {}", e),
    }
}
