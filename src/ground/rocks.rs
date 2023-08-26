use csv::ReaderBuilder;
use rocksdb::{WriteBatch, WriteOptions, DB};
use serde::{Serialize, Deserialize};
use serde_json::to_vec;  // For JSON serialization to Vec<u8>
use std::error::Error;

#[derive(Serialize, Deserialize)]
struct EventLog {
    agent_name: String,
    agent_id: String,
    event_action: String,
    utc_time: String,
    process_guid: String,
    process_id: String,
    image: String,
    file_version: String,
    description: String,
    product: String,
    company: String,
    original_file_name: String,
    command_line: String,
    current_directory: String,
    user: String,
    logon_guid: String,
    logon_id: String,
    terminal_session_id: String,
    integrity_level: String,
    hashes: String,
    parent_process_guid: String,
    parent_process_id: String,
    parent_image: String,
    parent_command_line: String,
    parent_user: String
}

// read csv files and save to RocksDB
// needs optimize code and Keys
fn main() -> Result<(), Box<dyn Error>> {
    // Open or create a RocksDB database
    let db = DB::open_default("/Users/dong-ju/Documents/My_code/elarocks/db")?;

    // Read CSV data with column names
    let csv_path = "/Users/dong-ju/Documents/My_code/elarocks/file/temp/event1_logs.csv";
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b'\t')  // Set the delimiter to a tab
        .from_path(csv_path)?;

    // Start a write batch for bulk writes
    let mut write_batch = WriteBatch::default();
    let write_options = WriteOptions::default();

    // Iterate over CSV rows and store in RocksDB
    for result in rdr.records() {
        let record = result?;

        // Create a struct from the CSV row
        let event = EventLog {
            agent_name: record.get(0).unwrap_or_default().to_string(),
            agent_id: record.get(1).unwrap_or_default().to_string(),
            event_action: record.get(2).unwrap_or_default().to_string(),
            utc_time: record.get(3).unwrap_or_default().to_string(),
            process_guid: record.get(4).unwrap_or_default().to_string(),
            process_id: record.get(5).unwrap_or_default().to_string(),
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
            logon_id: record.get(16).unwrap_or_default().to_string(),
            terminal_session_id: record.get(17).unwrap_or_default().to_string(),
            integrity_level: record.get(18).unwrap_or_default().to_string(),
            hashes: record.get(19).unwrap_or_default().to_string(),
            parent_process_guid: record.get(20).unwrap_or_default().to_string(),
            parent_process_id: record.get(21).unwrap_or_default().to_string(),
            parent_image: record.get(22).unwrap_or_default().to_string(),
            parent_command_line: record.get(23).unwrap_or_default().to_string(),
            parent_user: record.get(24).unwrap_or_default().to_string()
        };

        // Form the key and serialize the struct
        let key = format!("{}_{}", event.agent_id, event.utc_time);
        let serialized_value = to_vec(&event)?;

        // Add to the write batch
        write_batch.put(key.as_bytes(), &serialized_value);
    }

    // Commit the write batch to perform bulk writes
    db.write_opt(write_batch, &write_options)?;

    // Close the database
    Ok(())
}