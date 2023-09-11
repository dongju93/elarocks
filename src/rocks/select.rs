use rocksdb::{IteratorMode, DB};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let db = DB::open_default("/Users/dong-ju/Documents/My_code/elarocks/db")?;

    let partial_timestamp = "Process Create_2023-08-02 00:00:14.62300000";
    let base_key = partial_timestamp.as_bytes();

    // Create an iterator for the database starting from the specified partial timestamp
    let iter = db.iterator(IteratorMode::From(base_key, rocksdb::Direction::Forward));

    // Iterate over the data entries with keys
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
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error while iterating: {:?}", e);
            }
        }
    }

    Ok(())
}
