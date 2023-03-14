use serde_json::json;
use std::{
    env,
    sync::Arc,
};
use ton_client::{
    ClientConfig,
    ClientContext,
    net::{
        ParamsOfQuery,
        NetworkConfig,
        query,
    },
};

#[tokio::main]
async fn main() {
    let network = match env::var("TON_NETWORK_ADDRESS") {
        Ok(val) => val,
        Err(_) => "http://localhost/graphql".to_owned()
    };

    dbg!(network.clone());

    let context = Arc::new(
        ClientContext::new(ClientConfig {
            network: NetworkConfig {
                endpoints: Some(vec![network.to_owned()]),
                ..Default::default()
            },
            ..Default::default()
        })
        .unwrap(),
    );

    let output = query(
        context.clone(),
        ParamsOfQuery {
            query: r#"query{info{version}}"#.to_string(),
            variables: Some(json!({})),
            ..Default::default()
        }
    )
    .await
    .unwrap();
    dbg!(output.result);
}
