#![allow(deprecated)]

// External imports
use reqwest::header;
use serde_json::json;
use tokio;

// Internal module imports
mod env;
mod event;

// Constants & utility imports
use env::{ES_URL, ID, INDICES, PW, SIZE, TIMESTAMP};
use event::events::{EventOne, EventTwo};

// Functions
fn build_client() -> Result<reqwest::Client, reqwest::Error> {
    let basic_auth_header = format!("Basic {}", base64::encode(format!("{}:{}", ID, PW)));

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

fn build_query(event_code: &str) -> serde_json::Value {
    json!({
        "query": {
            "bool": {
                "must": [
                    { "match": {"event.code": event_code} },
                    { "match": {"event.module": "sysmon"} },
                    { "range": {"@timestamp": {"lt": TIMESTAMP}} }
                ]
            }
        },
        "size": SIZE
    })
}

async fn send_request(
    client: &reqwest::Client,
    query: &serde_json::Value,
    index: &str,
) -> Result<serde_json::Value, reqwest::Error> {
    client
        .post(&format!("{}/{}/_search", ES_URL, index))
        .json(query)
        .send()
        .await?
        .json()
        .await
}

async fn fetch_data_from_es(event_code: &str) -> Result<Vec<serde_json::Value>, reqwest::Error> {
    let client = build_client()?;
    let query = build_query(event_code);
    let mut all_results = Vec::new();

    for index in INDICES.iter() {
        all_results.push(send_request(&client, &query, index).await?);
    }

    Ok(all_results)
}

trait EventToCSV: Sized {
    fn parse(data: &serde_json::Value) -> Vec<Self>;
    fn write_to_csv(entries: &Vec<Self>, filename: &str) -> std::io::Result<()>;
}

impl EventToCSV for EventOne {
    fn parse(data: &serde_json::Value) -> Vec<Self> {
        let mut entries = Vec::new();

        if let Some(hits) = data["hits"]["hits"].as_array() {
            for hit in hits {
                if let Some(message) = hit["_source"]["message"].as_str() {
                    let mut entry = EventOne {
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
                        // println!("{:?}", segments); // Debug prints
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
                                "CurrentDirectory" => {
                                    entry.current_directory = Some(value.to_string())
                                }
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
                                "ParentProcessId" => {
                                    entry.parent_process_id = Some(value.to_string())
                                }
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

    fn write_to_csv(entries: &Vec<Self>, filename: &str) -> std::io::Result<()> {
        let mut wtr = csv::WriterBuilder::new()
            .delimiter(b'\t')
            .from_path(filename)?;
        for entry in entries {
            wtr.serialize(entry)?;
        }
        wtr.flush()?;
        Ok(())
    }
}

impl EventToCSV for EventTwo {
    fn parse(data: &serde_json::Value) -> Vec<Self> {
        let mut entries = Vec::new();

        if let Some(hits) = data["hits"]["hits"].as_array() {
            for hit in hits {
                if let Some(message) = hit["_source"]["message"].as_str() {
                    // println!("EventTwo raw message: {}", message);
                    let mut entry = EventTwo {
                        agent_name: None,
                        agent_id: None,
                        event_action: Some("File creation time changed".to_string()),
                        utc_time: None,
                        process_guid: None,
                        process_id: None,
                        image: None,
                        target_filename: None,
                        creation_utc_time: None,
                        previous_creation_utc_time: None,
                        user: None,
                    };

                    if let Some(agent_name) = hit["_source"]["agent"]["name"].as_str() {
                        entry.agent_name = Some(agent_name.to_string());
                    }

                    if let Some(agent_id) = hit["_source"]["agent"]["id"].as_str() {
                        entry.agent_id = Some(agent_id.to_string());
                    }

                    for part in message.split('\n') {
                        let segments: Vec<_> = part.splitn(2, ':').collect();
                        // println!("{:?}", segments); // Debug prints
                        if segments.len() == 2 {
                            let key = segments[0].trim();
                            let value = segments[1].trim();
                            match key {
                                "UtcTime" => entry.utc_time = Some(value.to_string()),
                                "ProcessGuid" => entry.process_guid = Some(value.to_string()),
                                "ProcessId" => entry.process_id = Some(value.to_string()),
                                "Image" => entry.image = Some(value.to_string()),
                                "TargetFilename" => entry.target_filename = Some(value.to_string()),
                                "CreationUtcTime" => {
                                    entry.creation_utc_time = Some(value.to_string())
                                }
                                "PreviousCreationUtcTime" => {
                                    entry.previous_creation_utc_time = Some(value.to_string())
                                }
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

    fn write_to_csv(entries: &Vec<Self>, filename: &str) -> std::io::Result<()> {
        let mut wtr = csv::WriterBuilder::new()
            .delimiter(b'\t')
            .from_path(filename)?;
        for entry in entries {
            wtr.serialize(entry)?;
        }
        wtr.flush()?;
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    for &event_code in &["1", "2"] {
        match fetch_data_from_es(event_code).await {
            Ok(datas) => {
                let filename = format!(
                    "/Users/dong-ju/Downloads/elacsv/event{}_logs.csv",
                    event_code
                );
                // println!("Raw data for event code {}: {:?}", event_code, datas);

                for data in &datas {
                    match event_code {
                        "1" => process_event_data::<EventOne>(data, &filename),
                        "2" => process_event_data::<EventTwo>(data, &filename),
                        _ => continue,
                    };
                }
            }
            Err(err) => eprintln!("Error: {:?}", err),
        }
    }
}

fn process_event_data<T: EventToCSV>(data: &serde_json::Value, filename: &str) {
    let entries = T::parse(data);
    println!("Number of entries: {}", entries.len());
    if let Err(e) = T::write_to_csv(&entries, filename) {
        eprintln!("Error writing to CSV: {:?}", e);
    }
}
