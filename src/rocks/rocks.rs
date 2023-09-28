#[allow(unused_imports)]
use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use csv::ReaderBuilder;
#[allow(unused_imports)]
use rocksdb::{
    OptimisticTransactionDB, OptimisticTransactionOptions, Options, SingleThreaded, WriteOptions,
};
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json::to_vec;
use std::error::Error;
use std::net::{IpAddr, Ipv4Addr};
#[allow(unused_imports)]
use std::ptr::null;
#[allow(unused_imports)]
use tokio_postgres::{Client, NoTls};
#[path = "../structs/mod.rs"]
mod structs;
use structs::eventTypes::*;
#[path = "../envs/mod.rs"]
mod envs;
use envs::db::*;

struct CsvConfig {
    csv_path: String,
    event_type: EventType,
    query: &'static str,
}

#[allow(dead_code)]
enum EventType {
    ProcessCreate,
    RegistryValueSet,
    NetworkConnection,
}

async fn save_keys_to_postgres(
    keys_to_save: &Vec<String>,
    query: &'static str,
) -> Result<(), Box<dyn Error>> {
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
        client.execute(query, &[&datetime_str]).await?;
    }
    client.batch_execute("COMMIT").await?;

    Ok(())
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
    let mut previous_utc_time = String::new();
    let mut keys_to_save = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let serialized_value = process_record(&record, &config.event_type)?;
        let naive_dt = NaiveDateTime::parse_from_str(
            record.get(3).unwrap_or_default(),
            "%Y-%m-%d %H:%M:%S%.3f",
        )?;
        let utc_time = Utc.from_utc_datetime(&naive_dt);

        if previous_utc_time != utc_time.to_string() {
            counter = 0;
            previous_utc_time = utc_time.to_string();
        }

        let formatted_time = utc_time.format("%Y-%m-%d %H:%M:%S%.3f").to_string();
        let key = format!(
            "{}_{}{}",
            record.get(2).unwrap_or_default(),
            formatted_time,
            format!("{:05}", counter)
        );

        keys_to_save.push(key.clone());
        counter += 1;
        transaction.put(key.as_bytes(), &serialized_value)?;
    }

    tokio::runtime::Runtime::new()?.block_on(save_keys_to_postgres(&keys_to_save, config.query))?;
    transaction.commit()?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let configs = vec![
        CsvConfig {
            csv_path: format!("{}{}", CSV_LOCA, "event13_logs.csv"),
            event_type: EventType::RegistryValueSet,
            query: DBINSE_REG,
        },
        // CsvConfig {
        //     csv_path: format!("{}{}", CSV_LOCA, "event1_logs.csv"),
        //     event_type: EventType::ProcessCreate,
        //     query: DBINSE_PRO,
        // },
        // CsvConfig {
        //     csv_path: format!("{}{}", CSV_LOCA, "event3_logs.csv"),
        //     event_type: EventType::NetworkConnection,
        //     query: DBINSE_NET,
        // },
    ];

    for config in &configs {
        process_csv(config)?;
    }

    Ok(())
}
