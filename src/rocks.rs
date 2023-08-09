use csv::ReaderBuilder;
use rocksdb::{WriteBatch, WriteOptions, DB};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Open or create a RocksDB database
    let db = DB::open_default("/Users/dong-ju/Documents/My_code/elarocks/db")?;

    // Read CSV data with column names
    let csv_path = "/Users/dong-ju/Documents/My_code/elarocks/file/event1_processcreate.csv";
    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(csv_path)?;

    // Start a write batch for bulk writes
    let mut write_batch = WriteBatch::default();
    let write_options = WriteOptions::default();

    // Read the headers (column names) from the CSV file
    let headers = rdr.headers()?.clone();

    // Iterate over CSV rows and store in RocksDB
    for result in rdr.records() {
        let record = result?;
        if let Some(timestamp) = record.get(0) {
            // Convert the timestamp to a string (as a base key)
            let base_key = timestamp;

            // Iterate over the columns and store in the write batch
            for (i, value) in record.iter().enumerate() {
                if i != 0 {
                    // Get the column name from the headers
                    let column = &headers[i];

                    let key = format!("{}_{}", base_key, column);
                    write_batch.put(key.as_bytes(), value.as_bytes());
                }
            }
        }
    }

    // Commit the write batch to perform bulk writes
    db.write_opt(write_batch, &write_options)?;

    // Close the database
    Ok(())
}
