use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use csv::ReaderBuilder;
use rocksdb::{WriteBatch, WriteOptions, DB};
use serde::{Deserialize, Serialize};
use serde_json::to_vec;
use std::error::Error;
use tokio_postgres::{Client, NoTls};

// Import Sysmon event structs
#[path = "../structs/mod.rs"]
mod structs;
use structs::eventTypes::*;
#[path = "../envs/mod.rs"]
mod envs;
use envs::env::{DBCONN, DBINSE_PRO, DBINSE_REG};

async fn save_keys_to_postgres(keys_to_save: &Vec<String>) -> Result<(), Box<dyn Error>> {
    let (client, connection) = tokio_postgres::connect(DBCONN, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    client.batch_execute("BEGIN").await?;
    for ktime in keys_to_save.iter() {
        let parts: Vec<&str> = ktime.split('_').collect();
        let datetime_str = parts.get(1).unwrap_or(&"");
        client.execute(DBINSE_REG, &[&datetime_str]).await?;
    }
    client.batch_execute("COMMIT").await?;

    Ok(())
}

// read csv files and save to RocksDB
fn main() -> Result<(), Box<dyn Error>> {
    let db = DB::open_default("/Users/dong-ju/Documents/My_code/elarocks/db")?;

    // Read CSV data with column names
    let csv_path = "/Users/dong-ju/Documents/My_code/elarocks/file/temp/event13_logs.csv";
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b'\t') // Set the delimiter to a tab
        .from_path(csv_path)?;

    // Start a write batch for bulk writes
    let mut write_batch = WriteBatch::default();
    let write_options = WriteOptions::default();

    let mut counter: u32 = 0;
    let mut previous_utc_time = String::new();
    let mut keys_to_save = Vec::new();

    // Iterate over CSV rows and store in RocksDB
    for result in rdr.records() {
        let record = result?;

        let naive_dt = NaiveDateTime::parse_from_str(
            record.get(3).unwrap_or_default(),
            "%Y-%m-%d %H:%M:%S%.3f",
        );
        let utc_time = naive_dt
            .map(|dt| DateTime::<Utc>::from_utc(dt, Utc))
            .unwrap_or_else(|_| Utc::now());

        // event 1
        // let event = ProcessCreateEvent {
        //     agent_name: record.get(0).unwrap_or_default().to_string(),
        //     agent_id: record.get(1).unwrap_or_default().to_string(),
        //     event_action: record.get(2).unwrap_or_default().to_string(),
        //     utc_time: utc_time,
        //     process_guid: record.get(4).unwrap_or_default().to_string(),
        //     process_id: record
        //         .get(5)
        //         .unwrap_or_default()
        //         .parse::<u32>()
        //         .unwrap_or(0),
        //     image: record.get(6).unwrap_or_default().to_string(),
        //     file_version: record.get(7).unwrap_or_default().to_string(),
        //     description: record.get(8).unwrap_or_default().to_string(),
        //     product: record.get(9).unwrap_or_default().to_string(),
        //     company: record.get(10).unwrap_or_default().to_string(),
        //     original_file_name: record.get(11).unwrap_or_default().to_string(),
        //     command_line: record.get(12).unwrap_or_default().to_string(),
        //     current_directory: record.get(13).unwrap_or_default().to_string(),
        //     user: record.get(14).unwrap_or_default().to_string(),
        //     logon_guid: record.get(15).unwrap_or_default().to_string(),
        //     logon_id: record
        //         .get(16)
        //         .unwrap_or_default()
        //         .parse::<u32>()
        //         .unwrap_or(0),
        //     terminal_session_id: record
        //         .get(17)
        //         .unwrap_or_default()
        //         .parse::<u32>()
        //         .unwrap_or(0),
        //     integrity_level: record.get(18).unwrap_or_default().to_string(),
        //     hashes: record.get(19).unwrap_or_default().to_string(),
        //     parent_process_guid: record.get(20).unwrap_or_default().to_string(),
        //     parent_process_id: record
        //         .get(21)
        //         .unwrap_or_default()
        //         .parse::<u32>()
        //         .unwrap_or(0),
        //     parent_image: record.get(22).unwrap_or_default().to_string(),
        //     parent_command_line: record.get(23).unwrap_or_default().to_string(),
        //     parent_user: record.get(24).unwrap_or_default().to_string(),
        // };

        // event 13
        let event = RegistryValueSetEvent {
            agent_name: record.get(0).unwrap_or_default().to_string(),
            agent_id: record.get(1).unwrap_or_default().to_string(),
            event_action: record.get(2).unwrap_or_default().to_string(),
            utc_time: utc_time,
            event_type: record.get(4).unwrap_or_default().to_string(),
            process_guid: record.get(5).unwrap_or_default().to_string(),
            process_id: record
                .get(6)
                .unwrap_or_default()
                .parse::<u32>()
                .unwrap_or(0),
            image: record.get(7).unwrap_or_default().to_string(),
            target_object: record.get(8).unwrap_or_default().to_string(),
            details: record.get(9).unwrap_or_default().to_string(),
            user: record.get(10).unwrap_or_default().to_string(),
        };

        // Check if utc_time has changed from the previous record
        if previous_utc_time != event.utc_time.to_string() {
            // If it has, reset the counter
            counter = 0;
            previous_utc_time = event.utc_time.to_string();
        }

        // Form the key with the counter and serialize the struct
        let formatted_time = event.utc_time.format("%Y-%m-%d %H:%M:%S%.3f").to_string();
        let key = format!(
            "{}_{}{}",
            event.event_action,
            formatted_time,
            format!("{:05}", counter)
        );

        // Add the time to the accumulator
        keys_to_save.push(key.clone());

        let serialized_value = to_vec(&event)?;

        // Increment the counter
        counter += 1;

        // Add to the write batch
        write_batch.put(key.as_bytes(), &serialized_value);
    }

    // pass time vac
    tokio::runtime::Runtime::new()?.block_on(save_keys_to_postgres(&keys_to_save))?;

    // Commit the write batch to perform bulk writes
    db.write_opt(write_batch, &write_options)?;

    Ok(())
}
