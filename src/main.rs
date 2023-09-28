// External Dependecys, import through Cargo.toml
use base64::{engine::general_purpose, Engine as _};
use rayon::prelude::*;
use reqwest::header;
use serde_json::json;
use tokio;

// Import Enviroments with secrect key (settings)
// Import Sysmon event structs
mod envs;
mod structs;

// use Imports
use envs::env::*;
use structs::events::*;

// Elasticearch client connection with bypass SSL (works with https)
fn build_client() -> Result<reqwest::Client, reqwest::Error> {
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

// Modify query
fn build_query(event_code: &str) -> serde_json::Value {
    json!({
        "query": {
            "bool": {
                "must": [
                    { "term": {"event.code": event_code} },
                    { "term": {"event.module": "sysmon"} },
                    { "range": {"@timestamp": {"gt": TIMESTAMP_STA, "lt": TIMESTAMP}} },
                    // 1. Used instead of wildcard when message's type is "match_only_text"
                    // { "query_string": {
                    //     "fields": ["message"],
                    //     "query": "*cFos* OR *Samsung*"
                    //   }
                    // },
                    // 2. Using wildcard when message's type is "keyword"
                    // {
                    //     "bool": {
                    //         "should": [
                    //             { "wildcard": { "message": "*.rmi*" } },
                    //             { "wildcard": { "message": "*.xml*" } }
                    //         ],
                    //         "minimum_should_match": 1
                    //     }
                    // },
                    // 3. Using match_phrase to search between characters
                    // {
                    //     "bool": {
                    //         "should": [
                    //             // { "match_phrase": { "message": "FileExts\\.rmi*" } },
                    //             { "match_phrase": { "message": "FileExts\\.xml*" } }
                    //         ],
                    //         "minimum_should_match": 1
                    //     }
                    // },
                ],
                // Combind with query_string and match_phrase
        //         "should": [
        //             {
        //                 "query_string": {
        //                     "fields": ["message"],
        //                     "query": "cFos OR *Samsung*"
        //                 }
        //             }, {
        //                 "bool": {
        //                     "should": [
        //                         // { "match_phrase": { "message": "FileExts\\.rmi*" } },
        //                         { "match_phrase": { "message": "FileExts\\.xml*" } }
        //                     ]
        //                 }
        //             }
        //         ],
        //         "minimum_should_match": 1
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
    println!("\n");
    for index in INDICES.iter() {
        all_results.push(send_request(&client, &query, index).await?);
        println!("Index {}", index)
    }

    Ok(all_results)
}

// after multi-thread processing
#[tokio::main]
async fn main() {
    let event_codes = [
        "1", "2", "3", "5", "7", "9", "10", "11", "12", "13", "14", "15", "17", "18", "22", "23",
        "25", "26",
    ];

    // Use rayon's `par_iter` to process each event code in parallel.
    event_codes.par_iter().for_each(|&event_code| {
        // Since `fetch_data_from_es` is async, we need to run it within a Tokio runtime.
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async {
            match fetch_data_from_es(event_code).await {
                Ok(datas) => {
                    let filename = format!("{}{}{}", SAVELOCATION, event_code, CSVNAME);
                    println!("Event {}", event_code);
                    for data in &datas {
                        match event_code {
                            "1" => process_event_data::<Event1>(data, &filename, SIZE),
                            "2" => process_event_data::<Event2>(data, &filename, SIZE),
                            "3" => process_event_data::<Event3>(data, &filename, SIZE),
                            "5" => process_event_data::<Event5>(data, &filename, SIZE),
                            "7" => process_event_data::<Event7>(data, &filename, SIZE),
                            "9" => process_event_data::<Event9>(data, &filename, SIZE),
                            "10" => process_event_data::<Event10>(data, &filename, SIZE),
                            "11" => process_event_data::<Event11>(data, &filename, SIZE),
                            "12" => process_event_data::<Event12>(data, &filename, SIZE),
                            "13" => process_event_data::<Event13>(data, &filename, SIZE),
                            "14" => process_event_data::<Event14>(data, &filename, SIZE),
                            "15" => process_event_data::<Event15>(data, &filename, SIZE),
                            "17" => process_event_data::<Event17>(data, &filename, SIZE),
                            "18" => process_event_data::<Event18>(data, &filename, SIZE),
                            "22" => process_event_data::<Event22>(data, &filename, SIZE),
                            "23" => process_event_data::<Event23>(data, &filename, SIZE),
                            "25" => process_event_data::<Event25>(data, &filename, SIZE),
                            "26" => process_event_data::<Event26>(data, &filename, SIZE),
                            _ => continue,
                        }
                    }
                }
                Err(err) => eprintln!("Error: {:?}", err),
            }
        });
    });
}
