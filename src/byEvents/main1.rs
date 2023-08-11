use reqwest::header;
use serde_json::json;
use tokio;

use crate::envs::env::*;
use crate::structs::events::Event1;

// Constants
const EVENT_CODE: &str = "1";

fn build_client() -> Result<reqwest::Client, reqwest::Error> {
    let auth_value = format!("{}:{}", ID, PW);
    let basic_auth_header = format!("Basic {}", base64::encode(auth_value));

    reqwest::Client::builder()
        .danger_accept_invalid_certs(true) // Bypass SSL verification (not recommended for production!)
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

fn build_query() -> serde_json::Value {
    json!({
        "query": {
            "bool": {
                "must": [
                    { "match": {"event.code": EVENT_CODE} },
                    { "match": {"event.module": "sysmon"} },
                    { "range": {"@timestamp": {"gt": TIMESTAMP_STA, "lt": TIMESTAMP}} },
                    // {"wildcard": {"message": "*h4ki032*"}}
                ]
            }
        },
          "size": SIZE
    })
}

async fn send_request(
    client: &reqwest::Client,
    query: &serde_json::Value,
) -> Result<serde_json::Value, reqwest::Error> {
    let url = format!("{}/{}/_search", ES_URL, INDEX);
    let response = client.post(&url).json(query).send().await?;
    response.json().await
}

async fn fetch_data_from_es() -> Result<serde_json::Value, reqwest::Error> {
    let client = build_client()?;
    let query = build_query();
    send_request(&client, &query).await
}

fn parse_output(data: &serde_json::Value) -> Vec<Event1> {
    let mut entries = Vec::new();

    if let Some(hits) = data["hits"]["hits"].as_array() {
        for hit in hits {
            if let Some(message) = hit["_source"]["message"].as_str() {
                let mut entry = Event1 {
                    agent_name: None,
                    agent_id: None,
                    event_action: Some("Process Create".to_string()),
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
                    parent_user: None,
                };

                if let Some(agent_name) = hit["_source"]["agent"]["name"].as_str() {
                    entry.agent_name = Some(agent_name.to_string());
                }

                if let Some(agent_id) = hit["_source"]["agent"]["id"].as_str() {
                    entry.agent_id = Some(agent_id.to_string());
                }

                for part in message.split('\n') {
                    let segments: Vec<_> = part.splitn(2, ':').collect();
                    println!("{:?}", segments); // Debug prints
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
                            "OriginalFileName" => {
                                entry.original_file_name = Some(value.to_string())
                            }
                            "CommandLine" => entry.command_line = Some(value.to_string()),
                            "CurrentDirectory" => entry.current_directory = Some(value.to_string()),
                            "User" => entry.user = Some(value.to_string()),
                            "LogonGuid" => entry.logon_guid = Some(value.to_string()),
                            "LogonId" => entry.logon_id = Some(value.to_string()),
                            "TerminalSessionId" => {
                                entry.terminal_session_id = Some(value.to_string())
                            }
                            "IntegrityLevel" => entry.integrity_level = Some(value.to_string()),
                            "Hashes" => entry.hashes = Some(value.to_string()),
                            "ParentProcessGuid" => {
                                entry.parent_process_guid = Some(value.to_string())
                            }
                            "ParentProcessId" => entry.parent_process_id = Some(value.to_string()),
                            "ParentImage" => entry.parent_image = Some(value.to_string()),
                            "ParentCommandLine" => {
                                entry.parent_command_line = Some(value.to_string())
                            }
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

fn write_to_csv(entries: Vec<Event1>, filename: &str) -> std::io::Result<()> {
    let mut wtr = csv::WriterBuilder::new()
        .delimiter(b'\t') // Set the delimiter to tab
        .from_path(filename)?;
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
            let filenames = format!("{}{}{}", SAVELOCATION, EVENT_CODE, CSVNAME);
            // Write the parsed data to a CSV file
            if let Err(e) = write_to_csv(
                entries,
                &filenames,
            ) {
                // if let Err(e) = write_to_csv(entries, "/Users/dong-ju/Dropbox/EINSIS/03. CODE/files/event1_processcreate_joe_pc_20230808_1200.csv") {
                eprintln!("Error writing to CSV: {:?}", e);
            }
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
        }
    }
}
