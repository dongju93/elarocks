
use reqwest::header;
use serde::Serialize;
use serde_json::json;
use tokio;
mod env;
use env::{ES_URL, ID, INDEX, PW};

// Constants
const EVENT_CODE: &str = "3";
const TIMESTAMP: &str = "2023-08-08T03:00:00.000Z";
const SIZE: usize = 10000000;

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

#[derive(Serialize)] // We're using the serde crate's Serialize trait to help with CSV writing
struct EventThree {
    agent_name: Option<String>,
    agent_id: Option<String>,
    event_action: Option<String>,
    utc_time: Option<String>,
    process_guid: Option<String>,
    process_id: Option<String>,
    image: Option<String>,
    user: Option<String>,
    initiated: Option<String>,
    protocol: Option<String>,
    source_is_ipv6: Option<String>,
    source_ip: Option<String>,
    source_hostname: Option<String>,
    source_port: Option<String>,
    source_port_name: Option<String>,
    destination_is_ipv6: Option<String>,
    destination_ip: Option<String>,
    destination_hostname: Option<String>,
    destination_port: Option<String>,
    destination_port_name: Option<String>,
}

fn parse_output(data: &serde_json::Value) -> Vec<EventThree> {
    let mut entries = Vec::new();

    if let Some(hits) = data["hits"]["hits"].as_array() {
        for hit in hits {
            if let Some(message) = hit["_source"]["message"].as_str() {
                let mut entry = EventThree {
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
                    println!("{:?}", segments); // Debug prints
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
                            "SourcePortName" => entry.source_port_name = Some(value.to_string()),
                            "DestinationIsIpv6" => {
                                entry.destination_is_ipv6 = Some(value.to_string())
                            }
                            "DestinationIp" => entry.destination_ip = Some(value.to_string()),
                            "DestinationHostname" => {
                                entry.destination_hostname = Some(value.to_string())
                            }
                            "DestinationPort" => entry.destination_port = Some(value.to_string()),
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

fn write_to_csv(entries: Vec<EventThree>, filename: &str) -> std::io::Result<()> {
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
            let entries: Vec<EventThree> = parse_output(&data);

            // Write the parsed data to a CSV file
            if let Err(e) = write_to_csv(
                entries,
                "C:/Users/samsung/Downloads/csvfiles/event3_networkconn_joe_pc_20230808_1200.csv",
            ) {
                // if let Err(e) = write_to_csv(entries, "/Users/dong-ju/Dropbox/EINSIS/03. CODE/files/event3_networkconn_joe_pc_20230808_1200.csv") {
                eprintln!("Error writing to CSV: {:?}", e);
            }
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
        }
    }
}
