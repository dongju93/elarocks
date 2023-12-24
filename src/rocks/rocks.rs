#![allow(unused_imports)]

use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use csv::ReaderBuilder;
use rocksdb::{
    OptimisticTransactionDB, OptimisticTransactionOptions, Options, SingleThreaded, WriteOptions,
};
use serde::{Deserialize, Serialize};
use serde_json::to_vec;
use std::error::Error;
use std::net::{IpAddr, Ipv4Addr};
use std::ptr::null;
// use tokio_postgres::{Client, NoTls};
use std::env;
#[path = "../structs/mod.rs"]
mod structs;
use structs::eventTypes::*;
#[path = "../envs/mod.rs"]
mod envs;
use envs::db::*;

struct CsvConfig {
    csv_path: String,
    event_type: EventType,
    // query: &'static str,
}

#[allow(dead_code)]
enum EventType {
    ProcessCreate,
    RegistryValueSet,
    NetworkConnection,
}

fn process_record(
    record: &csv::StringRecord,
    event_type: &EventType,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let naive_dt =
        NaiveDateTime::parse_from_str(record.get(3).unwrap_or_default(), "%Y-%m-%d %H:%M:%S%.3f")?;
    let utc_time = Utc.from_utc_datetime(&naive_dt);

    match event_type {
        EventType::ProcessCreate => {
            let event = ProcessCreateEvent {
                agent_name: record.get(0).unwrap_or_default().to_string(),
                agent_id: record.get(1).unwrap_or_default().to_string(),
                event_action: record.get(2).unwrap_or_default().to_string(),
                utc_time: utc_time,
                process_guid: record.get(4).unwrap_or_default().to_string(),
                process_id: record
                    .get(5)
                    .unwrap_or_default()
                    .parse::<u32>()
                    .unwrap_or(0),
                image: record.get(6).unwrap_or_default().to_string(),
                file_version: record.get(7).unwrap_or_default().to_string(),
                description: record.get(8).unwrap_or_default().to_string(),
                product: record.get(9).unwrap_or_default().to_string(),
                company: record.get(10).unwrap_or_default().to_string(),
                original_file_name: record.get(11).unwrap_or_default().to_string(),
                command_line: record.get(12).unwrap_or_default().to_string(),
                current_directory: record.get(13).unwrap_or_default().to_string(),
                user: record.get(14).unwrap_or_default().to_string(),
                logon_guid: record.get(15).unwrap_or_default().to_string(),
                logon_id: record
                    .get(16)
                    .unwrap_or_default()
                    .parse::<u32>()
                    .unwrap_or(0),
                terminal_session_id: record
                    .get(17)
                    .unwrap_or_default()
                    .parse::<u32>()
                    .unwrap_or(0),
                integrity_level: record.get(18).unwrap_or_default().to_string(),
                hashes: record.get(19).unwrap_or_default().to_string(),
                parent_process_guid: record.get(20).unwrap_or_default().to_string(),
                parent_process_id: record
                    .get(21)
                    .unwrap_or_default()
                    .parse::<u32>()
                    .unwrap_or(0),
                parent_image: record.get(22).unwrap_or_default().to_string(),
                parent_command_line: record.get(23).unwrap_or_default().to_string(),
                parent_user: record.get(24).unwrap_or_default().to_string(),
            };
            to_vec(&event).map_err(|e| Box::new(e) as Box<dyn Error>)
        }
        EventType::RegistryValueSet => {
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
            to_vec(&event).map_err(|e| Box::new(e) as Box<dyn Error>)
        }
        EventType::NetworkConnection => {
            let event = NetworkConnectionEvent {
                agent_name: record.get(0).unwrap_or_default().to_string(),
                agent_id: record.get(1).unwrap_or_default().to_string(),
                event_action: record.get(2).unwrap_or_default().to_string(),
                utc_time: utc_time,
                process_guid: record.get(4).unwrap_or_default().to_string(),
                process_id: record
                    .get(5)
                    .unwrap_or_default()
                    .parse::<u32>()
                    .unwrap_or(0),
                image: record.get(6).unwrap_or_default().to_string(),
                user: record.get(7).unwrap_or_default().to_string(),
                protocol: record.get(8).unwrap_or_default().to_string(),
                initiated: record
                    .get(9)
                    .unwrap_or_default()
                    .parse::<bool>()
                    .unwrap_or(false),
                source_is_ipv6: record
                    .get(10)
                    .unwrap_or_default()
                    .parse::<bool>()
                    .unwrap_or(false),
                source_ip: record
                    .get(11)
                    .unwrap_or_default()
                    .parse::<IpAddr>()
                    .unwrap_or(IpAddr::V4(Ipv4Addr::UNSPECIFIED)),
                source_hostname: record.get(12).unwrap_or_default().to_string(),
                source_port: record
                    .get(13)
                    .unwrap_or_default()
                    .parse::<u16>()
                    .unwrap_or(0),
                source_port_name: record.get(14).unwrap_or_default().to_string(),
                destination_is_ipv6: record
                    .get(15)
                    .unwrap_or_default()
                    .parse::<bool>()
                    .unwrap_or(false),
                destination_ip: record
                    .get(16)
                    .unwrap_or_default()
                    .parse::<IpAddr>()
                    .unwrap_or(IpAddr::V4(Ipv4Addr::UNSPECIFIED)),
                destination_hostname: record.get(17).unwrap_or_default().to_string(),
                destination_port: record
                    .get(18)
                    .unwrap_or_default()
                    .parse::<u16>()
                    .unwrap_or(0),
                destination_port_name: record.get(19).unwrap_or_default().to_string(),
            };
            to_vec(&event).map_err(|e| Box::new(e) as Box<dyn Error>)
        } // _ => Err(Box::from("Unknown event type")),
    }
}

fn process_csv(config: &CsvConfig) -> Result<(), Box<dyn Error>> {
    let mut opts = Options::default();
    opts.create_if_missing(true);
    let db: OptimisticTransactionDB<SingleThreaded> =
        OptimisticTransactionDB::open(&opts, DB_LOCA)?;

    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b'\t')
        .from_path(&config.csv_path)?;

    let transaction = db.transaction();

    let mut counter: u32 = 0;
    let mut previous_naive_dt =
        NaiveDateTime::from_timestamp_opt(0, 0).expect("Initial timestamp should be valid");

    for result in rdr.records() {
        let record = result?;
        let serialized_value = process_record(&record, &config.event_type)?;
        let naive_dt = match NaiveDateTime::parse_from_str(
            record.get(3).unwrap_or_default(),
            "%Y-%m-%d %H:%M:%S%.3f",
        ) {
            Ok(dt) => dt,
            Err(e) => {
                eprintln!("Error parsing datetime: {}", e);
                continue; // Skip this record or handle as needed
            }
        };

        if naive_dt == previous_naive_dt {
            counter += 1;
        } else {
            counter = 0;
            previous_naive_dt = naive_dt;
        }

        let additional_nanos = counter % 1_000_000; // to keep it within nanosecond range
        let adjusted_naive_dt = naive_dt
            .checked_add_signed(chrono::Duration::nanoseconds(additional_nanos as i64))
            .expect("Adjusted time should be valid");

        let utc_time = Utc.from_utc_datetime(&adjusted_naive_dt);
        let epoch_time_nanos = match utc_time.timestamp_nanos_opt() {
            Some(nanos) => nanos,
            None => {
                eprintln!("Warning: Timestamp is out of range for nanosecond precision.");
                continue; // Skip this record or handle as needed
            }
        };

        let key = format!("{}_{}", record.get(2).unwrap_or_default(), epoch_time_nanos);

        println!("{}", key);

        transaction.put(key.as_bytes(), &serialized_value)?;
    }

    transaction.commit()?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} [Event Type Number]", args[0]);
        return Ok(());
    }

    let event_type_number = &args[1];
    let event_type = match event_type_number.as_str() {
        "1" => EventType::ProcessCreate,
        "3" => EventType::NetworkConnection,
        "13" => EventType::RegistryValueSet,
        _ => {
            eprintln!("Invalid event type number: {}", event_type_number);
            return Ok(());
        }
    };

    let csv_path = format!("{}event{}_logs.csv", CSV_LOCA, event_type_number);

    let config = CsvConfig {
        csv_path,
        event_type,
    };

    process_csv(&config)?;

    Ok(())
}
