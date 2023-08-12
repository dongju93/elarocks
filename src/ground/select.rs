use rocksdb::{IteratorMode, DB};
use std::error::Error;

// Show selected Keys with Values within RocksDB
fn main() -> Result<(), Box<dyn Error>> {
    // Open the RocksDB database
    let db = DB::open_default("/Users/dong-ju/Documents/My_code/elarocks/db")?;

    // Example: Select data for a specific timestamp (wildcard match)
    let partial_timestamp = "2023-08-07 12:05:02.936";
    let base_key = partial_timestamp.as_bytes();

    // Create an iterator for the database starting from the specified partial timestamp
    let iter = db.iterator(IteratorMode::From(base_key, rocksdb::Direction::Forward));

    // Iterate over the data entries with keys matching the wildcard
    for item in iter {
        match item {
            Ok((key, value)) => {
                // Convert the key to a string
                let key_str = String::from_utf8(key.to_vec())?;

                // Check if the key includes the desired partial timestamp
                if key_str.contains(partial_timestamp) {
                    let value_str = String::from_utf8(value.to_vec())?;
                    println!("Key: {}, Value: {}", key_str, value_str);
                } else {
                    // If the key does not contain the desired partial timestamp, break the loop
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error while iterating: {:?}", e);
                // Handle the error as needed
            }
        }
    }

    Ok(())
}
