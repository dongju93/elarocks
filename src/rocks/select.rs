use chrono::{NaiveDateTime, TimeZone, Utc};
use rocksdb::{Direction, IteratorMode, DB};
use serde_json::{Map, Value};
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut total_count = 0;
    let mut written_count = 0;
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        return Err("Please provide start_key and end_key as arguments".into());
    }

    let start_key = args[1].as_bytes();
    let end_key = args[2].as_bytes();
    let direction = args.get(3).map_or(Direction::Forward, |arg| {
        if arg == "last" {
            Direction::Reverse
        } else {
            Direction::Forward
        }
    });
    let input_max_records = args
        .get(4)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(usize::MAX);
    let cursor = args.get(5).and_then(|s| s.parse::<i64>().ok());
    let image_contain = args.get(6).map(String::as_str); // Optional argument
    let pid_match = args.get(7).and_then(|s| s.parse::<u32>().ok());

    let iter_mode = match direction {
        Direction::Forward => IteratorMode::From(start_key, Direction::Forward),
        Direction::Reverse => IteratorMode::From(end_key, Direction::Reverse),
    };

    let db = DB::open_default("/Users/dong-ju/Documents/My_code/elarocks/db")?;
    let mut iter = db.iterator(iter_mode);

    let count_iter_mode = IteratorMode::From(start_key, Direction::Forward);
    let mut count_iter = db.iterator(count_iter_mode);
    for item in &mut count_iter {
        if let Ok((key, value)) = item {
            if key.as_ref() > end_key {
                break;
            }
            let value_str = String::from_utf8(value.to_vec())?;
            let json: Map<String, Value> = serde_json::from_str(&value_str)?;
            if image_contain.map_or(true, |img| {
                json.get("image")
                    .and_then(|v| v.as_str())
                    .map_or(false, |image| image.contains(img))
            }) && pid_match.map_or(true, |pid| {
                json.get("process_id")
                    .and_then(|v| v.as_u64())
                    .map_or(false, |id| id as u32 == pid)
            }) {
                total_count += 1;
            }
        }
    }

    if let Some(cursor_time) = cursor {
        while let Some(result) = iter.next() {
            let (key, _) = result?;

            let key_time = key_to_epoch(&String::from_utf8(key.to_vec())?)?;
            let condition = match direction {
                Direction::Forward => key_time >= cursor_time,
                Direction::Reverse => key_time <= cursor_time,
            };

            if condition {
                break;
            }
        }
    }

    let mut has_next_page = false;
    let mut has_previous_page = false;
    let mut last_key = None;
    let mut first_key = None;
    let mut is_cursor_condition_met = cursor.is_none(); // true if cursor is not provided

    print!("[");
    for item in &mut iter {
        if let Ok((key, value)) = item {
            // Check if the key is within the specified range
            match direction {
                Direction::Reverse => {
                    if key.as_ref() < start_key {
                        break;
                    }
                }
                Direction::Forward => {
                    if key.as_ref() > end_key {
                        break;
                    }
                }
            }

            // Process the value
            let value_str = String::from_utf8(value.to_vec())?;
            let json: Map<String, Value> = serde_json::from_str(&value_str)?;

            // Check for image and pid match
            let image_match = image_contain.map_or(true, |img| {
                json.get("image")
                    .and_then(|v| v.as_str())
                    .map_or(false, |image| image.contains(img))
            });

            let pid_match = pid_match.map_or(true, |pid| {
                json.get("process_id")
                    .and_then(|v| v.as_u64())
                    .map_or(false, |id| id as u32 == pid)
            });

            if image_match && pid_match {
                // Check if the cursor condition is met
                if !is_cursor_condition_met {
                    let key_time = key_to_epoch(&String::from_utf8(key.to_vec())?)?;
                    is_cursor_condition_met = match direction {
                        Direction::Forward => key_time >= cursor.unwrap_or(0),
                        Direction::Reverse => key_time <= cursor.unwrap_or(i64::MAX),
                    };
                }

                // Check if the record is within the pagination window
                if is_cursor_condition_met && written_count < input_max_records {
                    let epoch_time = key_to_epoch(&String::from_utf8(key.to_vec())?)?;
                    print!(
                        "{{\"cursor\": \"{}\", \"node\": {}}},",
                        epoch_time, value_str
                    );
                    written_count += 1;

                    if first_key.is_none() {
                        first_key = Some(key.clone());
                    }
                    last_key = Some(key.clone());
                }
            }
        }
    }

    if let Some(last_key) = last_key {
        has_previous_page = if cursor.is_some() {
            match direction {
                Direction::Forward => has_more_records(
                    &db,
                    start_key,
                    first_key.as_deref().unwrap_or(start_key),
                    image_contain,
                    pid_match,
                    Direction::Forward,
                ),
                Direction::Reverse => has_more_records(
                    &db,
                    end_key,
                    first_key.as_deref().unwrap_or(end_key),
                    image_contain,
                    pid_match,
                    Direction::Reverse,
                ),
            }
        } else {
            false
        };

        has_next_page = match direction {
            Direction::Forward => has_more_records(
                &db,
                last_key.as_ref(),
                end_key,
                image_contain,
                pid_match,
                Direction::Forward,
            ),
            Direction::Reverse => has_more_records(
                &db,
                last_key.as_ref(),
                start_key,
                image_contain,
                pid_match,
                Direction::Reverse,
            ),
        };
    }

    println!(
        "{{\"total_count\": {}, \"written_count\": {}, \"has_next_page\": {}, \"has_previous_page\": {}}}]",
        total_count, written_count, has_next_page, has_previous_page
    );

    Ok(())
}

fn key_to_epoch(key: &str) -> Result<i64, Box<dyn Error>> {
    let parts: Vec<&str> = key.split('_').collect();
    let timestamp_str = parts.get(1).ok_or("Timestamp not found in key")?;

    let naive_dt = NaiveDateTime::parse_from_str(timestamp_str, "%Y-%m-%d %H:%M:%S%.f")?;

    let datetime = Utc.from_utc_datetime(&naive_dt);

    match datetime.timestamp_nanos_opt() {
        Some(nanos) => Ok(nanos),
        None => Err("Failed to convert to nanoseconds".into()),
    }
}

fn has_more_records(
    db: &DB,
    key: &[u8],
    boundary_key: &[u8],
    image_contain: Option<&str>,
    pid_match: Option<u32>,
    direction: Direction,
) -> bool {
    let iter_mode = IteratorMode::From(key, direction);
    let mut iter = db.iterator(iter_mode);

    while let Some(Ok((k, value))) = iter.next() {
        let k_slice = k.as_ref(); // Convert Box<[u8]> to &[u8]
        let within_bounds = match direction {
            Direction::Forward => k_slice >= key && k_slice < boundary_key,
            Direction::Reverse => k_slice <= key && k_slice > boundary_key,
        };

        if !within_bounds {
            break;
        }

        let value_str = match String::from_utf8(value.to_vec()) {
            Ok(val) => val,
            Err(_) => continue,
        };

        let json: Map<String, Value> = match serde_json::from_str(&value_str) {
            Ok(json) => json,
            Err(_) => continue,
        };

        let image_match = image_contain.map_or(true, |img| {
            json.get("image")
                .and_then(|v| v.as_str())
                .map_or(false, |image| image.contains(img))
        });

        let pid_match_condition = pid_match.map_or(true, |pid| {
            json.get("process_id")
                .and_then(|v| v.as_u64())
                .map_or(false, |id| id as u32 == pid)
        });

        if image_match && pid_match_condition {
            return true;
        }
    }

    false
}
