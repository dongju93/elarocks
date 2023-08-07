use reqwest::header;
use reqwest::Client;
use rocksdb::{Options, DB};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, json, to_string};
use tokio;
use csv::Writer;
mod env;
use env::{INDEX, ES_URL, ID, PW};
use chrono::{DateTime, Utc, Duration, FixedOffset};

// Constants
const EVENT_CODE: &str = "1";
const TIMESTAMP: &str = "2023-08-07T03:05:11.628Z";
const SIZE: usize = 10000000;

fn build_client() -> Result<reqwest::Client, reqwest::Error> {
    let auth_value = format!("{}:{}", ID, PW);
    let basic_auth_header = format!("Basic {}", base64::encode(auth_value));

    reqwest::Client::builder()
        .danger_accept_invalid_certs(true) // Bypass SSL verification (not recommended for production!)
        .default_headers({
            let mut headers = header::HeaderMap::new();
            headers.insert(header::AUTHORIZATION, header::HeaderValue::from_str(&basic_auth_header).unwrap());
            headers
        })
        .build()
}

fn build_query() -> serde_json::Value {
    json!({
        "query": {
            "bool": {
              "must": [
                { "match": {"event.code": EVENT_CODE} },
                { "range": {"@timestamp": {"lt": TIMESTAMP}} }
              ]
            }
          },
          "size": SIZE
    })
}

async fn send_request(client: &reqwest::Client, query: &serde_json::Value) -> Result<serde_json::Value, reqwest::Error> {
    let url = format!("{}/{}/_search", ES_URL, INDEX);
    let response = client
        .post(&url)
        .json(query)
        .send()
        .await?;
    response.json().await
}


async fn fetch_data_from_es() -> Result<serde_json::Value, reqwest::Error> {
    let client = build_client()?;
    let query = build_query();
    send_request(&client, &query).await
}

#[derive(Serialize)] // We're using the serde crate's Serialize trait to help with CSV writing
struct EventTwo {
    timestamp: Option<String>,
    event_type: Option<String>,
    rule_name: Option<String>,
    utc_time: Option<String>,
    process_guid: Option<String>,
    process_id: Option<String>,
    image: Option<String>,
    target_filename: Option<String>,
    creation_utc_time: Option<String>,
    previous_creation_utc_time: Option<String>,
    user: Option<String>
}

fn parse_output(data: &serde_json::Value) -> Vec<EventTwo> {
    let mut entries = Vec::new();

    if let Some(hits) = data["hits"]["hits"].as_array() {
        for hit in hits {
            if let Some(message) = hit["_source"]["message"].as_str() {
                let mut entry = EventTwo {
                    timestamp: None,
                    event_type: Some("File creation time changed".to_string()),
                    rule_name: None,
                    utc_time: None,
                    process_guid: None,
                    process_id: None,
                    image: None,
                    target_filename: None,
                    creation_utc_time: None,
                    previous_creation_utc_time: None,
                    user: None
                };
                
                if let Some(ts) = hit["_source"]["@timestamp"].as_str() {
                    if let Ok(datetime) = DateTime::parse_from_rfc3339(ts) {
                        let adjusted_time = (datetime.with_timezone(&Utc) + Duration::hours(9)).to_string();
                        entry.timestamp = Some(adjusted_time.replace("UTC", "")); // Remove "UTC" from timestamp
                    }
                }
                
                for part in message.split('\n') {
                    let segments: Vec<_> = part.splitn(2, ':').collect();
                    println!("{:?}", segments);  // Debug prints
                    if segments.len() == 2 {
                        let key = segments[0].trim();
                        let value = segments[1].trim();
                        match key {
                            "RuleName" => entry.rule_name = Some(value.to_string()),
                            "UtcTime" => entry.utc_time = Some(value.to_string()),
                            "ProcessGuid" => entry.process_guid = Some(value.to_string()),
                            "ProcessId" => entry.process_id = Some(value.to_string()),
                            "Image" => entry.image = Some(value.to_string()),
                            "TargetFilename" => entry.target_filename = Some(value.to_string()),
                            "CreationUtcTime" => entry.creation_utc_time = Some(value.to_string()),
                            "PreviousCreationUtcTime" => entry.previous_creation_utc_time = Some(value.to_string()),
                            "User" => entry.user = Some(value.to_string()),


                            _ => {}
                        }
                    }
                }
                
                entries.push(entry);
            }
        }
    }

    entries
}

fn write_to_csv(entries: Vec<EventTwo>, filename: &str) -> std::io::Result<()> {
    let mut wtr = Writer::from_path(filename)?;
    for entry in entries {
        wtr.serialize(entry)?;
    }
    wtr.flush()?;
    Ok(())
}


#[tokio::main]
async fn main() {
    match fetch_data_from_es().await {
        Ok(data) => {
            let entries = parse_output(&data);
            
            // Write the parsed data to a CSV file
            // if let Err(e) = write_to_csv(entries, "C:/Users/spdlq/Dropbox/EINSIS/03. CODE/files/event1_processcreate.csv") {
            if let Err(e) = write_to_csv(entries, "/Users/dong-ju/Dropbox/EINSIS/03. CODE/files/event2_filecreate.csv") {
                eprintln!("Error writing to CSV: {:?}", e);
            }
        },
        Err(err) => {
            eprintln!("Error: {:?}", err);
        }
    }
}
