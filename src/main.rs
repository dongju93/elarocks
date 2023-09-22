#![allow(deprecated, clippy::wildcard_imports)]

// Import Sysmon event structs
mod events;
// Run with configuration file
mod config;

use crate::{config::Config, events::*};
use anyhow::Result;
// External Dependecys, import through Cargo.toml
use rayon::prelude::*;
use reqwest::header;
use serde_json::json;
use std::env as senv;
use std::process::exit;

// after multi-thread processing
#[tokio::main]
async fn main() -> Result<()> {
    let config: Config = Config::new(parse().as_deref())?;
    let size = config.query_size;

    let event_codes = [
        "1", "2", "3", "5", "7", "9", "10", "11", "12", "13", "14", "15", "17", "18", "22", "23",
        "25", "26",
    ];

    // Use rayon's `par_iter` to process each event code in parallel.
    event_codes.par_iter().for_each(|&event_code| {
        // Since `fetch_data_from_es` is async, we need to run it within a Tokio runtime.
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async {
            match fetch_data_from_es(event_code, config.clone()).await {
                Ok(datas) => {
                    let filename =
                        format!("{}{event_code}{}", config.save_location, config.csv_name);
                    println!("Event {event_code}");
                    for data in &datas {
                        match event_code {
                            "1" => process_event_data::<Event1>(data, &filename, size),
                            "2" => process_event_data::<Event2>(data, &filename, size),
                            "3" => process_event_data::<Event3>(data, &filename, size),
                            "5" => process_event_data::<Event5>(data, &filename, size),
                            "7" => process_event_data::<Event7>(data, &filename, size),
                            "9" => process_event_data::<Event9>(data, &filename, size),
                            "10" => process_event_data::<Event10>(data, &filename, size),
                            "11" => process_event_data::<Event11>(data, &filename, size),
                            "12" => process_event_data::<Event12>(data, &filename, size),
                            "13" => process_event_data::<Event13>(data, &filename, size),
                            "14" => process_event_data::<Event14>(data, &filename, size),
                            "15" => process_event_data::<Event15>(data, &filename, size),
                            "17" => process_event_data::<Event17>(data, &filename, size),
                            "18" => process_event_data::<Event18>(data, &filename, size),
                            "22" => process_event_data::<Event22>(data, &filename, size),
                            "23" => process_event_data::<Event23>(data, &filename, size),
                            "25" => process_event_data::<Event25>(data, &filename, size),
                            "26" => process_event_data::<Event26>(data, &filename, size),
                            _ => continue,
                        }
                    }
                }
                Err(err) => eprintln!("Error: {err:?}"),
            }
        });
    });
    Ok(())
}

// Elastic search client connection with bypass SSL (works with https)
fn build_client(id: &str, pw: &str) -> Result<reqwest::Client, reqwest::Error> {
    let basic_auth_header = format!("Basic {}", base64::encode(format!("{id}:{pw}")));

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

// Modify query
fn build_query(
    event_code: &str,
    start_time: &str,
    end_time: &str,
    size: usize,
) -> serde_json::Value {
    json!({
        "query": {
            "bool": {
                "must": [
                    { "term": {"event.code": event_code} },
                    { "term": {"event.module": "sysmon"} },
                    { "range": {"@timestamp": {"gt": start_time, "lt": end_time}} },
                ],
            }
        },
        "size": size
    })
}

// Send query with "_search" option
async fn send_request(
    client: &reqwest::Client,
    query: &serde_json::Value,
    index: &str,
    es_url: &str,
) -> Result<serde_json::Value, reqwest::Error> {
    client
        .post(&format!("{es_url}/{index}/_search"))
        .json(query)
        .send()
        .await?
        .json()
        .await
}

// Query multiple Index with event_code
async fn fetch_data_from_es(
    event_code: &str,
    config: Config,
) -> Result<Vec<serde_json::Value>, reqwest::Error> {
    let client = build_client(&config.es_id, &config.es_pw)?;
    let query = build_query(
        event_code,
        &config.start_time,
        &config.end_time,
        config.query_size,
    );
    let mut all_results = Vec::new();
    println!("\n");
    for index in &config.indices {
        all_results.push(send_request(&client, &query, index, &config.es_url).await?);
        println!("Index {index}");
    }

    Ok(all_results)
}

fn parse() -> Option<String> {
    let mut args = senv::args();
    args.next()?;

    let arg = args.next()?;
    if args.next().is_some() {
        eprintln!("too many arguments");
        exit(1);
    }

    Some(arg)
}
