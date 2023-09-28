#![allow(dead_code)]

use super::env::*;
use base64::{engine::general_purpose, Engine as _};
use reqwest::header;
use serde_json::json;

// change EVE_CODE to exact byEvents number
pub const INDEX: &str = ".ds-winlogbeat-8.8.2-2023.08.06-000001";
const EVE_CODE: &str = "13";

// change TIMESTAMP if needed
// const TIMESTAMP_STA: &str = "2023-08-01T00:00:00.000Z";
// const TIMESTAMP: &str = "2023-08-01T00:00:00.000Z";

pub fn build_query() -> serde_json::Value {
    json!({
        "query": {
            "bool": {
                "must": [
                    { "term": {"event.code": EVE_CODE} },
                    { "term": {"event.module": "sysmon"} },
                    { "range": {"@timestamp": {"gt": TIMESTAMP_STA, "lt": TIMESTAMP}} },
                    // Used instead of wildcard when message's type is "match_only_text"
                    // { "query_string": {
                    //     "fields": ["message"],
                    //     "query": "\\.rmi"
                    //   }
                    // },
                    // Using wildcard when message's type is "keyword"
                    // {
                    //     "bool": {
                    //         "should": [
                    //             { "wildcard": { "message": "*.rmi*" } },
                    //             { "wildcard": { "message": "*.xml*" } }
                    //         ],
                    //         "minimum_should_match": 1
                    //     }
                    // },
                    // Using match_phrase to search between characters
                    // {
                    //     "bool": {
                    //         "should": [
                    //             { "match_phrase": { "message": "FileExts\\.rmi*" } },
                    //             { "match_phrase": { "message": "FileExts\\.xml*" } }
                    //         ],
                    //         "minimum_should_match": 1
                    //     }
                    // },
                ]
            }
        },
        "size": SIZE
    })
}

pub fn build_client() -> Result<reqwest::Client, reqwest::Error> {
    let auth_value = format!("{}:{}", ID, PW);
    let auth_value_bytes = auth_value.as_bytes();
    let encoded: String = general_purpose::STANDARD_NO_PAD.encode(auth_value_bytes);
    let basic_auth_header = format!("Basic {}", encoded);

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
