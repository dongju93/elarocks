#![allow(dead_code)]

use super::env::*;
use serde_json::json;
use reqwest::header;

// change EVE_CODE to exact byEvents number
pub const INDEX: &str = ".ds-winlogbeat-8.8.2-2023.08.06-000001";
const EVE_CODE: &str = "2";

// change TIMESTAMP if needed
// const TIMESTAMP_STA: &str = "2023-08-01T00:00:00.000Z";
// const TIMESTAMP: &str = "2023-08-01T00:00:00.000Z";

pub fn build_query() -> serde_json::Value {
    json!({
        "query": {
            "bool": {
                "must": [
                    { "match": {"event.code": EVE_CODE} },
                    { "match": {"event.module": "sysmon"} },
                    { "range": {"@timestamp": {"gt": TIMESTAMP_STA, "lt": TIMESTAMP}} },
                    // {"wildcard": {"message": "*h4ki032*"}}
                ]
            }
        },
          "size": SIZE
    })
}

pub fn build_client() -> Result<reqwest::Client, reqwest::Error> {
    let auth_value = format!("{}:{}", ID, PW);
    let basic_auth_header = format!("Basic {}", base64::encode(auth_value));

    reqwest::Client::builder()
        .danger_accept_invalid_certs(true) 
        .default_headers({
            let mut headers = header::HeaderMap::new();
            headers.insert(
                header::AUTHORIZATION,
                header::HeaderValue::from_str(&basic_auth_header).unwrap(),
            );
            headers
        })
        .build()
}

pub async fn send_request(
    client: &reqwest::Client,
    query: &serde_json::Value,
) -> Result<serde_json::Value, reqwest::Error> {
    let url = format!("{}/{}/_search", ES_URL, INDEX);
    let response = client.post(&url).json(query).send().await?;
    response.json().await
}

pub async fn fetch_data_from_es() -> Result<serde_json::Value, reqwest::Error> {
    let client = build_client()?;
    let query = build_query();
    send_request(&client, &query).await
}
