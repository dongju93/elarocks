#![allow(deprecated)]

// External Dependecys, import through Cargo.toml
use reqwest::header;
use serde_json::json;
use tokio;

// Import Enviroments with secrect key (settings)
#[path = "envs/mod.rs"]
mod envs;
// Import Sysmon event structs
#[path = "structs/mod.rs"]
mod structs;

// use Imports
use envs::env::*;
use structs::events::*;

// Elasticearch client connection with bypass SSL (works with https)
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

// Modify query
fn build_query(event_code: &str) -> serde_json::Value {
    json!({
        "query": {
            "bool": {
                "must": [
                    { "match": {"event.code": event_code} },
                    { "match": {"event.module": "sysmon"} },
                    { "range": {"@timestamp": {"gt": TIMESTAMP_STA, "lt": TIMESTAMP}} },
                    // {"wildcard": {"message": "*h4ki032*"}}
                ]
            }
        },
        "size": SIZE
    })
}

// Send query with "_search" option
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

// Query multiple Index with event_code
async fn fetch_data_from_es(event_code: &str) -> Result<Vec<serde_json::Value>, reqwest::Error> {
    let client = build_client()?;
    let query = build_query(event_code);
    let mut all_results = Vec::new();

    for index in INDICES.iter() {
        all_results.push(send_request(&client, &query, index).await?);
    }

    Ok(all_results)
}

// A common interface for a group of types.
trait EventToCSV: Sized {
    fn parse(data: &serde_json::Value) -> Vec<Self>;
    fn write_to_csv(entries: &Vec<Self>, filename: &str) -> std::io::Result<()>;
}

// Parse json(response data) and make .csv files
impl EventToCSV for Event1 {
    fn parse(data: &serde_json::Value) -> Vec<Self> {
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

impl EventToCSV for Event2 {
    fn parse(data: &serde_json::Value) -> Vec<Self> {
        let mut entries = Vec::new();

        if let Some(hits) = data["hits"]["hits"].as_array() {
            for hit in hits {
                if let Some(message) = hit["_source"]["message"].as_str() {
                    // println!("Event2 raw message: {}", message);
                    let mut entry = Event2 {
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

impl EventToCSV for Event3 {
    fn parse(data: &serde_json::Value) -> Vec<Self> {
        let mut entries = Vec::new();

        if let Some(hits) = data["hits"]["hits"].as_array() {
            for hit in hits {
                if let Some(message) = hit["_source"]["message"].as_str() {
                    // println!("Event2 raw message: {}", message);
                    let mut entry = Event3 {
                        agent_name: None,
                        agent_id: None,
                        event_action: Some("Network connection detected".to_string()),
                        utc_time: None,
                        process_guid: None,
                        process_id: None,
                        image: None,
                        user: None,
                        protocol: None,
                        initiated: None,
                        source_is_ipv6: None,
                        source_ip: None,
                        source_hostname: None,
                        source_port: None,
                        source_port_name: None,
                        destination_is_ipv6: None,
                        destination_ip: None,
                        destination_hostname: None,
                        destination_port: None,
                        destination_port_name: None,
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
                                "User" => entry.image = Some(value.to_string()),
                                "Protocol" => entry.protocol = Some(value.to_string()),
                                "Initiated" => entry.initiated = Some(value.to_string()),
                                "SourceIsIpv6" => entry.source_is_ipv6 = Some(value.to_string()),
                                "SourceIp" => entry.source_ip = Some(value.to_string()),
                                "SourceHostname" => entry.source_hostname = Some(value.to_string()),
                                "SourcePort" => entry.source_port = Some(value.to_string()),
                                "SourcePortName" => {
                                    entry.source_port_name = Some(value.to_string())
                                }
                                "DestinationIsIpv6" => {
                                    entry.destination_is_ipv6 = Some(value.to_string())
                                }
                                "DestinationIp" => entry.destination_ip = Some(value.to_string()),
                                "DestinationHostname" => {
                                    entry.destination_hostname = Some(value.to_string())
                                }
                                "DestinationPort" => {
                                    entry.destination_port = Some(value.to_string())
                                }
                                "DestinationPortName" => {
                                    entry.destination_port_name = Some(value.to_string())
                                }
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

impl EventToCSV for Event5 {
    fn parse(data: &serde_json::Value) -> Vec<Self> {
        let mut entries = Vec::new();

        if let Some(hits) = data["hits"]["hits"].as_array() {
            for hit in hits {
                if let Some(message) = hit["_source"]["message"].as_str() {
                    // println!("Event2 raw message: {}", message);
                    let mut entry = Event5 {
                        agent_name: None,
                        agent_id: None,
                        event_action: Some("Process terminated".to_string()),
                        utc_time: None,
                        process_guid: None,
                        process_id: None,
                        image: None,
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

impl EventToCSV for Event7 {
    fn parse(data: &serde_json::Value) -> Vec<Self> {
        let mut entries = Vec::new();

        if let Some(hits) = data["hits"]["hits"].as_array() {
            for hit in hits {
                if let Some(message) = hit["_source"]["message"].as_str() {
                    // println!("Event2 raw message: {}", message);
                    let mut entry = Event7 {
                        agent_name: None,
                        agent_id: None,
                        event_action: Some("Image loaded".to_string()),
                        utc_time: None,
                        process_guid: None,
                        process_id: None,
                        image: None,
                        image_loaded: None,
                        file_version: None,
                        description: None,
                        product: None,
                        company: None,
                        original_file_name: None,
                        hashes: None,
                        signed: None,
                        signature: None,
                        signature_status: None,
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
                                "ImageLoaded" => entry.image_loaded = Some(value.to_string()),
                                "FileVersion" => entry.file_version = Some(value.to_string()),
                                "Description" => entry.description = Some(value.to_string()),
                                "Product" => entry.product = Some(value.to_string()),
                                "Company" => entry.company = Some(value.to_string()),
                                "OriginalFileName" => {
                                    entry.original_file_name = Some(value.to_string())
                                }
                                "Hashes" => entry.hashes = Some(value.to_string()),
                                "Signed" => entry.signed = Some(value.to_string()),
                                "Signature" => entry.signature = Some(value.to_string()),
                                "SignatureStatus" => {
                                    entry.signature_status = Some(value.to_string())
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

impl EventToCSV for Event11 {
    fn parse(data: &serde_json::Value) -> Vec<Self> {
        let mut entries = Vec::new();

        if let Some(hits) = data["hits"]["hits"].as_array() {
            for hit in hits {
                if let Some(message) = hit["_source"]["message"].as_str() {
                    // println!("Event2 raw message: {}", message);
                    let mut entry = Event11 {
                        agent_name: None,
                        agent_id: None,
                        event_action: Some("File created".to_string()),
                        utc_time: None,
                        process_guid: None,
                        process_id: None,
                        image: None,
                        target_filename: None,
                        creation_utc_time: None,
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

impl EventToCSV for Event13 {
    fn parse(data: &serde_json::Value) -> Vec<Self> {
        let mut entries = Vec::new();

        if let Some(hits) = data["hits"]["hits"].as_array() {
            for hit in hits {
                if let Some(message) = hit["_source"]["message"].as_str() {
                    // println!("Event2 raw message: {}", message);
                    let mut entry = Event13 {
                        agent_name: None,
                        agent_id: None,
                        event_action: Some("Registry value set".to_string()),
                        event_type: None,
                        utc_time: None,
                        process_guid: None,
                        process_id: None,
                        image: None,
                        target_object: None,
                        details: None,
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
                                "EventType" => entry.event_type = Some(value.to_string()),
                                "UtcTime" => entry.utc_time = Some(value.to_string()),
                                "ProcessGuid" => entry.process_guid = Some(value.to_string()),
                                "ProcessId" => entry.process_id = Some(value.to_string()),
                                "Image" => entry.image = Some(value.to_string()),
                                "TargetObject" => entry.target_object = Some(value.to_string()),
                                "Details" => entry.details = Some(value.to_string()),
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

impl EventToCSV for Event14 {
    fn parse(data: &serde_json::Value) -> Vec<Self> {
        let mut entries = Vec::new();

        if let Some(hits) = data["hits"]["hits"].as_array() {
            for hit in hits {
                if let Some(message) = hit["_source"]["message"].as_str() {
                    // println!("Event2 raw message: {}", message);
                    let mut entry = Event14 {
                        agent_name: None,
                        agent_id: None,
                        event_action: Some("Registry value set".to_string()),
                        event_type: None,
                        utc_time: None,
                        process_guid: None,
                        process_id: None,
                        image: None,
                        target_object: None,
                        new_name: None,
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
                                "EventType" => entry.event_type = Some(value.to_string()),
                                "UtcTime" => entry.utc_time = Some(value.to_string()),
                                "ProcessGuid" => entry.process_guid = Some(value.to_string()),
                                "ProcessId" => entry.process_id = Some(value.to_string()),
                                "Image" => entry.image = Some(value.to_string()),
                                "TargetObject" => entry.target_object = Some(value.to_string()),
                                "NewName" => entry.new_name = Some(value.to_string()),
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

impl EventToCSV for Event15 {
    fn parse(data: &serde_json::Value) -> Vec<Self> {
        let mut entries = Vec::new();

        if let Some(hits) = data["hits"]["hits"].as_array() {
            for hit in hits {
                if let Some(message) = hit["_source"]["message"].as_str() {
                    // println!("Event2 raw message: {}", message);
                    let mut entry = Event15 {
                        agent_name: None,
                        agent_id: None,
                        event_action: Some("File stream created".to_string()),
                        utc_time: None,
                        process_guid: None,
                        process_id: None,
                        image: None,
                        target_filename: None,
                        creation_utc_time: None,
                        hash: None,
                        contents: None,
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
                                "Hash" => entry.hash = Some(value.to_string()),
                                "Contents" => entry.contents = Some(value.to_string()),
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

impl EventToCSV for Event17 {
    fn parse(data: &serde_json::Value) -> Vec<Self> {
        let mut entries = Vec::new();

        if let Some(hits) = data["hits"]["hits"].as_array() {
            for hit in hits {
                if let Some(message) = hit["_source"]["message"].as_str() {
                    // println!("Event2 raw message: {}", message);
                    let mut entry = Event17 {
                        agent_name: None,
                        agent_id: None,
                        event_action: Some("Pipe Created".to_string()),
                        event_type: None,
                        utc_time: None,
                        process_guid: None,
                        process_id: None,
                        pipe_name: None,
                        image: None,
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
                                "EventType" => entry.event_type = Some(value.to_string()),
                                "UtcTime" => entry.utc_time = Some(value.to_string()),
                                "ProcessGuid" => entry.process_guid = Some(value.to_string()),
                                "ProcessId" => entry.process_id = Some(value.to_string()),
                                "PipeName" => entry.pipe_name = Some(value.to_string()),
                                "Image" => entry.image = Some(value.to_string()),
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

impl EventToCSV for Event22 {
    fn parse(data: &serde_json::Value) -> Vec<Self> {
        let mut entries = Vec::new();

        if let Some(hits) = data["hits"]["hits"].as_array() {
            for hit in hits {
                if let Some(message) = hit["_source"]["message"].as_str() {
                    // println!("Event2 raw message: {}", message);
                    let mut entry = Event22 {
                        agent_name: None,
                        agent_id: None,
                        event_action: Some("Dns query".to_string()),
                        utc_time: None,
                        process_guid: None,
                        process_id: None,
                        query_name: None,
                        query_status: None,
                        query_results: None,
                        image: None,
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
                                "QueryName" => entry.query_name = Some(value.to_string()),
                                "QueryStatus" => entry.query_status = Some(value.to_string()),
                                "QueryResults" => entry.query_results = Some(value.to_string()),
                                "Image" => entry.image = Some(value.to_string()),
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

impl EventToCSV for Event23 {
    fn parse(data: &serde_json::Value) -> Vec<Self> {
        let mut entries = Vec::new();

        if let Some(hits) = data["hits"]["hits"].as_array() {
            for hit in hits {
                if let Some(message) = hit["_source"]["message"].as_str() {
                    // println!("Event2 raw message: {}", message);
                    let mut entry = Event23 {
                        agent_name: None,
                        agent_id: None,
                        event_action: Some("File Delete archived,".to_string()),
                        utc_time: None,
                        process_guid: None,
                        process_id: None,
                        user: None,
                        image: None,
                        target_filename: None,
                        hashes: None,
                        is_executable: None,
                        archived: None,
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
                                "User" => entry.user = Some(value.to_string()),
                                "Image" => entry.image = Some(value.to_string()),
                                "TargetFilename" => entry.target_filename = Some(value.to_string()),
                                "Hashes" => entry.hashes = Some(value.to_string()),
                                "IsExecutable" => entry.is_executable = Some(value.to_string()),
                                "Archived" => entry.archived = Some(value.to_string()),
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

impl EventToCSV for Event25 {
    fn parse(data: &serde_json::Value) -> Vec<Self> {
        let mut entries = Vec::new();

        if let Some(hits) = data["hits"]["hits"].as_array() {
            for hit in hits {
                if let Some(message) = hit["_source"]["message"].as_str() {
                    // println!("Event2 raw message: {}", message);
                    let mut entry = Event25 {
                        agent_name: None,
                        agent_id: None,
                        event_action: Some("Process Tampering,".to_string()),
                        utc_time: None,
                        process_guid: None,
                        process_id: None,
                        image: None,
                        types: None,
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
                                "Type" => entry.types = Some(value.to_string()),
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

impl EventToCSV for Event26 {
    fn parse(data: &serde_json::Value) -> Vec<Self> {
        let mut entries = Vec::new();

        if let Some(hits) = data["hits"]["hits"].as_array() {
            for hit in hits {
                if let Some(message) = hit["_source"]["message"].as_str() {
                    // println!("Event2 raw message: {}", message);
                    let mut entry = Event26 {
                        agent_name: None,
                        agent_id: None,
                        event_action: Some("File Delete logged,".to_string()),
                        utc_time: None,
                        process_guid: None,
                        process_id: None,
                        user: None,
                        image: None,
                        target_filename: None,
                        hashes: None,
                        is_executable: None,
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
                                "User" => entry.user = Some(value.to_string()),
                                "Image" => entry.image = Some(value.to_string()),
                                "TargetFilename" => entry.target_filename = Some(value.to_string()),
                                "Hashes" => entry.hashes = Some(value.to_string()),
                                "IsExecutable" => entry.is_executable = Some(value.to_string()),
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

// Excute functions with argumentss
#[tokio::main]
async fn main() {
    for &event_code in &[
        "1", "2", "3", "5", "7", "11", "13", "14", "15", "17", "22", "23", "25", "26",
    ] {
        match fetch_data_from_es(event_code).await {
            Ok(datas) => {
                let filename = format!("{}{}{}", SAVELOCATION, event_code, CSVNAME);
                // println!("Raw data for event code {}: {:?}", event_code, datas);

                for data in &datas {
                    match event_code {
                        "1" => process_event_data::<Event1>(data, &filename),
                        "2" => process_event_data::<Event2>(data, &filename),
                        "3" => process_event_data::<Event3>(data, &filename),
                        "5" => process_event_data::<Event5>(data, &filename),
                        "7" => process_event_data::<Event7>(data, &filename),
                        "11" => process_event_data::<Event11>(data, &filename),
                        "13" => process_event_data::<Event13>(data, &filename),
                        "14" => process_event_data::<Event14>(data, &filename),
                        "15" => process_event_data::<Event15>(data, &filename),
                        "17" => process_event_data::<Event17>(data, &filename),
                        "22" => process_event_data::<Event22>(data, &filename),
                        "23" => process_event_data::<Event23>(data, &filename),
                        "25" => process_event_data::<Event25>(data, &filename),
                        "26" => process_event_data::<Event26>(data, &filename),

                        _ => continue,
                    };
                }
            }
            Err(err) => eprintln!("Error: {:?}", err),
        }
    }
}

// Printout counts each events
fn process_event_data<T: EventToCSV>(data: &serde_json::Value, filename: &str) {
    let entries = T::parse(data);
    println!("Number of entries: {}", entries.len());
    if let Err(e) = T::write_to_csv(&entries, filename) {
        eprintln!("Error writing to CSV: {:?}", e);
    }
}
