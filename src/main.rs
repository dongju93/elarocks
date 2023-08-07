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
struct EventOne {
    timestamp: Option<String>,
    event_type: Option<String>,
    rule_name: Option<String>,
    utc_time: Option<String>,
    process_guid: Option<String>,
    process_id: Option<String>,
    image: Option<String>,
    file_version: Option<String>,
    description: Option<String>,
    product: Option<String>,
    company: Option<String>,
    original_file_name: Option<String>,
    command_line: Option<String>,
    current_directory: Option<String>,
    user: Option<String>,
    logon_guid: Option<String>,
    logon_id: Option<String>,
    terminal_session_id: Option<String>,
    integrity_level: Option<String>,
    hashes: Option<String>,
    parent_process_guid: Option<String>,
    parent_process_id: Option<String>,
    parent_image: Option<String>,
    parent_command_line: Option<String>,
    parent_user: Option<String>
}

fn parse_output(data: &serde_json::Value) -> Vec<EventOne> {
    let mut entries = Vec::new();

    if let Some(hits) = data["hits"]["hits"].as_array() {
        for hit in hits {
            if let Some(message) = hit["_source"]["message"].as_str() {
                let mut entry = EventOne {
                    timestamp: None,
                    event_type: Some("Process Create".to_string()),
                    rule_name: None,
                    utc_time: None,
                    process_guid: None,
                    process_id: None,
                    image: None,
                    file_version: None,
                    description: None,
                    product: None,
                    company: None,
                    original_file_name: None,
                    command_line: None,
                    current_directory: None,
                    user: None,
                    logon_guid: None,
                    logon_id: None,
                    terminal_session_id: None,
                    integrity_level: None,
                    hashes: None,
                    parent_process_guid: None,
                    parent_process_id: None,
                    parent_image: None,
                    parent_command_line: None,
                    parent_user: None
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
                            "UtcTime" => entry.utc_time = Some(value.to_string()),
                            "ProcessGuid" => entry.process_guid = Some(value.to_string()),
                            "ProcessId" => entry.process_id = Some(value.to_string()),
                            "Image" => entry.image = Some(value.to_string()),
                            "FileVersion" => entry.file_version = Some(value.to_string()),
                            "Description" => entry.description = Some(value.to_string()),
                            "Product" => entry.product = Some(value.to_string()),
                            "Company" => entry.company = Some(value.to_string()),
                            "OriginalFileName" => entry.original_file_name = Some(value.to_string()),
                            "CommandLine" => entry.command_line = Some(value.to_string()),
                            "CurrentDirectory" => entry.current_directory = Some(value.to_string()),
                            "User" => entry.user = Some(value.to_string()),
                            "LogonGuid" => entry.logon_guid = Some(value.to_string()),
                            "LogonId" => entry.logon_id = Some(value.to_string()),
                            "TerminalSessionId" => entry.terminal_session_id = Some(value.to_string()),
                            "IntegrityLevel" => entry.integrity_level = Some(value.to_string()),
                            "Hashes" => entry.hashes = Some(value.to_string()),
                            "ParentProcessGuid" => entry.parent_process_guid = Some(value.to_string()),
                            "ParentProcessId" => entry.parent_process_id = Some(value.to_string()),
                            "ParentImage" => entry.parent_image = Some(value.to_string()),
                            "ParentCommandLine" => entry.parent_command_line = Some(value.to_string()),
                            "ParentUser" => entry.parent_user = Some(value.to_string()),
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

fn write_to_csv(entries: Vec<EventOne>, filename: &str) -> std::io::Result<()> {
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
            if let Err(e) = write_to_csv(entries, "/Users/dong-ju/Dropbox/EINSIS/03. CODE/files/event1_processcreate.csv") {
                eprintln!("Error writing to CSV: {:?}", e);
            }
        },
        Err(err) => {
            eprintln!("Error: {:?}", err);
        }
    }
}
