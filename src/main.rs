use reqwest::header;
use reqwest::Client;
use rocksdb::{Options, DB};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, json, to_string};
use tokio;

mod env;

use env::{INDEX, ES_URL, ID, PW};

async fn fetch_data_from_es() -> Result<serde_json::Value, reqwest::Error> {
    let auth_value = format!("{}:{}", ID, PW);
    let basic_auth_header = format!("Basic {}", base64::encode(auth_value));

    let client = Client::builder()
        .danger_accept_invalid_certs(true) // Bypass SSL verification (not recommended for production!)
        .build()?;

    let url = format!("{}/{}/_search", ES_URL, INDEX);

    let query = json!({
        "query": {
            "bool": {
                "must": [
                    { "match": {"event.code": "1"} }
                ]
            }
        },
        "size": 1
    });

    let response = client
        .post(&url)
        .header(header::AUTHORIZATION, basic_auth_header)
        .json(&query)
        .send()
        .await?;

    Ok(response.json().await?)
}

#[tokio::main]
async fn main() {
    match fetch_data_from_es().await {
        Ok(data) => {
            if let Some(hits) = data["hits"]["hits"].as_array() {
                for hit in hits {
                    if let Some(message) = hit["_source"]["message"].as_str() {
                        for part in message.split('\t') {
                            println!("{}", part);
                        }
                    }
                }
            }
        },
        Err(err) => {
            eprintln!("Error: {:?}", err);
        }
    }
}