use chrono::{NaiveDateTime, TimeZone, Utc};
use rocksdb::{Direction, IteratorMode, ReadOptions, DB};
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    // at least give 2 args, stat/end time
    if args.len() < 3 {
        eprintln!("Usage: nano_select <start_key> <end_key>");
        return Err("Insufficient arguments".into());
    }

    // TODO
    // split input keys with event name, date time
    let original_start_key = &args[1];
    let original_end_key = &args[2];
    let search_direction = args.get(3).map(String::as_str);
    let max_print_count: usize = args
        .get(4)
        .and_then(|s| s.parse().ok())
        .unwrap_or(usize::MAX);
    let cursor_key: Option<Vec<u8>> = args.get(5).map(|s| s.clone().into_bytes());
    let image_contains: Option<String> = args.get(6).cloned();
    let process_id_exact: Option<u32> = args.get(7).and_then(|s| s.parse().ok());

    // split _ event name
    let event_name_start_part = original_start_key
        .split('_')
        .nth(0)
        .ok_or("Failed to extract datetime from start key")?;
    let event_name_end_part = original_end_key
        .split('_')
        .nth(0)
        .ok_or("Failed to extract datetime from end key")?;

    // split _ nano seconds data time
    let datetime_start_part = original_start_key
        .split('_')
        .nth(1)
        .ok_or("Failed to extract datetime from start key")?;
    let datetime_end_part = original_end_key
        .split('_')
        .nth(1)
        .ok_or("Failed to extract datetime from end key")?;

    // convert nano seconds date time string to formatted UTC
    let start_naive_dt =
        NaiveDateTime::parse_from_str(datetime_start_part, "%Y-%m-%d %H:%M:%S%.f")?;
    let end_naive_dt = NaiveDateTime::parse_from_str(datetime_end_part, "%Y-%m-%d %H:%M:%S%.f")?;

    // making UTC ti epoch time to for keys
    let epoch_start_time = Utc
        .from_utc_datetime(&start_naive_dt)
        .timestamp_nanos_opt()
        .ok_or("Failed to convert start datetime to nanoseconds")?;
    let epoch_end_time = Utc
        .from_utc_datetime(&end_naive_dt)
        .timestamp_nanos_opt()
        .ok_or("Failed to convert end datetime to nanoseconds")?;

    // merge with names and epoch time to real key
    let start_key = format!("{}_{}", event_name_start_part, epoch_start_time).into_bytes();
    let end_key = format!("{}_{}", event_name_end_part, epoch_end_time).into_bytes();

    // input with 'first' will forward search (start → end)
    // input with 'last' will reverse search (end → start)
    let iterator_mode = match search_direction {
        Some("first") => IteratorMode::From(start_key.as_slice(), Direction::Forward),
        Some("last") => IteratorMode::From(end_key.as_slice(), Direction::Reverse),
        _ => IteratorMode::From(start_key.as_slice(), Direction::Forward), // default to forward if not specified
    };

    let mut read_options = ReadOptions::default();
    // limit lower and upper data for searching
    read_options.set_iterate_lower_bound(start_key.as_slice());
    read_options.set_iterate_upper_bound(end_key.as_slice());

    let db = DB::open_default("/Users/dong-ju/Documents/My_code/elarocks/nano_db")?;

    let iterator = db.iterator_opt(iterator_mode, read_options);

    let mut total_count = 0;
    let mut printed_count = 0;
    let mut start_cursor = None;
    let mut end_cursor = None;
    let start_printing = cursor_key
        .as_ref()
        .map(|cursor| cursor.is_empty())
        .unwrap_or(true); // Start printing(stdout) immediately if cursor is null or ""
    let mut found_cursor = false;
    let mut has_previous_page = false;
    let mut has_next_page = false;
    let is_reverse_search = search_direction == Some("last");

    print!("[");
    for item in iterator {
        match item {
            Ok((key, value)) => {
                // key and value to string
                let value_str = String::from_utf8_lossy(&value).to_string();
                let key_str = String::from_utf8_lossy(&key).to_string();
                // Deserialize value into JSON object
                let json: serde_json::Value = serde_json::from_str(&value_str)?;

                // in forward search and current key is bigger than end_key then stop
                if !is_reverse_search && key.as_ref() > end_key.as_slice() {
                    break;
                }

                // in reverse search and end_key is is bigger than current key then stop
                if is_reverse_search && key.as_ref() < start_key.as_slice() {
                    break;
                }

                // total count is increase filter is true
                if apply_filters(&json, &image_contains, &process_id_exact) {
                    total_count += 1;

                    // if the current key is the cursor key then skip for data duplication
                    if let Some(cursor) = &cursor_key {
                        if key.as_ref() == cursor.as_slice() {
                            found_cursor = true;
                            continue;
                        }
                    }

                    if found_cursor || start_printing {
                        if printed_count < max_print_count {
                            // Set the end_cursor on the first item printed
                            if printed_count == 0 {
                                start_cursor = Some(key_str.clone());
                            }

                            // Always update the end_cursor to the last item printed
                            end_cursor = Some(key_str.clone());
                            print!("{{\"cursor\": \"{}\", \"node\": {}}},", key_str, value_str);
                            printed_count += 1;
                        }
                    }
                }
            }

            Err(e) => {
                eprintln!("{{Error reading from RocksDB: {}}},", e);
                // Handle the error, e.g., break the loop or continue to the next item
            }
        }
    }

    // Check for previous page
    if let Some(cursor) = &cursor_key {
        let cursor_key_bytes = cursor.as_slice();

        if is_reverse_search {
            // Reverse search: Iterate from end_key to cursor and check for data existence
            let iter = db.iterator(IteratorMode::From(end_key.as_slice(), Direction::Reverse));
            has_previous_page = false;
            if !cursor.is_empty() {
                for result in iter {
                    match result {
                        Ok((key, value)) => {
                            let value_str = String::from_utf8_lossy(&value).to_string();
                            let json: serde_json::Value = serde_json::from_str(&value_str)?;

                            if apply_filters(&json, &image_contains, &process_id_exact)
                                && key.as_ref() > cursor_key_bytes
                                && key.as_ref() != end_key.as_slice()
                            {
                                has_previous_page = true;
                                break; // Break on finding the first key greater than the cursor
                            }
                        }
                        Err(_) => continue, // Optionally handle errors or continue
                    }
                }
            }
        } else {
            // Forward search: Iterate from start_key to cursor and check for data existence
            let iter = db.iterator(IteratorMode::From(start_key.as_slice(), Direction::Forward));
            has_previous_page = false;
            if !cursor.is_empty() {
                for result in iter {
                    match result {
                        Ok((key, value)) => {
                            let value_str = String::from_utf8_lossy(&value).to_string();
                            let json: serde_json::Value = serde_json::from_str(&value_str)?;

                            if apply_filters(&json, &image_contains, &process_id_exact)
                                && key.as_ref() < cursor_key_bytes
                                && key.as_ref() != start_key.as_slice()
                            {
                                has_previous_page = true;
                                break; // Break on finding the first key less than the cursor
                            }
                        }
                        Err(_) => continue, // Optionally handle errors or continue
                    }
                }
            }
        }
    }

    let end_cursor_bytes = end_cursor.as_ref().map(|s| s.as_bytes()).unwrap_or(&[]);
    if is_reverse_search {
        // Reverse search: Iterate from end_key to cursor and check for data existence
        let iter = db.iterator(IteratorMode::From(end_cursor_bytes, Direction::Reverse));
        has_next_page = false;
        for result in iter {
            match result {
                Ok((key, value)) => {
                    let value_str = String::from_utf8_lossy(&value).to_string();
                    let json: serde_json::Value = serde_json::from_str(&value_str)?;

                    if apply_filters(&json, &image_contains, &process_id_exact)
                        && key.as_ref() > start_key.as_slice()
                        && key.as_ref() != end_cursor_bytes
                    {
                        has_next_page = true;
                        break; // Break on finding the first key greater than the cursor
                    }
                }
                Err(_) => continue, // Optionally handle errors or continue
            }
        }
    } else {
        // Forward search: Iterate from start_key to cursor and check for data existence
        let iter = db.iterator(IteratorMode::From(end_cursor_bytes, Direction::Forward));
        has_next_page = false;
        for result in iter {
            match result {
                Ok((key, value)) => {
                    let value_str = String::from_utf8_lossy(&value).to_string();
                    let json: serde_json::Value = serde_json::from_str(&value_str)?;

                    if apply_filters(&json, &image_contains, &process_id_exact)
                        && key.as_ref() < end_key.as_slice()
                        && key.as_ref() != end_cursor_bytes
                    {
                        has_next_page = true;
                        break; // Break on finding the first key less than the cursor
                    }
                }
                Err(_) => continue, // Optionally handle errors or continue
            }
        }
    }

    println!("{{\"total_count\": {}, \"counts\": {}, \"start_cursor\": \"{}\", \"end_cursor\": \"{}\", \"has_previous_page\": {}, \"has_next_page\": {}}}]", total_count, printed_count, start_cursor.as_deref().unwrap_or(""), end_cursor.as_deref().unwrap_or(""), has_previous_page, has_next_page);

    Ok(())
}

fn apply_filters(
    json: &serde_json::Value,
    image_contains: &Option<String>,
    process_id_exact: &Option<u32>,
) -> bool {
    // Convert empty strings to None
    let image_contains = if let Some(ref image_substr) = image_contains {
        if image_substr.is_empty() {
            None
        } else {
            Some(image_substr)
        }
    } else {
        None
    };

    // Convert empty strings to None
    let process_id_exact = if let Some(pid) = process_id_exact {
        Some(*pid)
    } else {
        None
    };

    let image_condition = match image_contains {
        Some(image_substr) => json["image"]
            .as_str()
            .map_or(false, |img| img.contains(image_substr)),
        None => true, // No image condition specified, so it's always true
    };

    let process_id_condition = match process_id_exact {
        Some(pid) => json["process_id"]
            .as_u64()
            .map_or(false, |p| p == pid as u64),
        None => true, // No process_id condition specified, so it's always true
    };

    image_condition && process_id_condition
}
