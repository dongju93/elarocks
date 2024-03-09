// use crate::envs::db::DB_LOCA;
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

                    // skip cursor or cursor is true (default)
                    if found_cursor || start_printing {
                        // printed count is less than max count
                        if printed_count < max_print_count {
                            // if printed first then set start_cursor as current key (clone)
                            // first entry of key
                            if printed_count == 0 {
                                start_cursor = Some(key_str.clone());
                            }

                            // keep update end_cursor as current key (clone)
                            // last entry of key
                            end_cursor = Some(key_str.clone());
                            print!("{{\"cursor\": \"{}\", \"node\": {}}},", key_str, value_str);
                            printed_count += 1;
                        }
                    }
                }
            }

            Err(e) => {
                // keep struct as json format
                eprintln!("{{Error reading from RocksDB: {}}},", e);
            }
        }
    }

    // check for there is previous data (page)
    if let Some(cursor) = &cursor_key {
        // convert cursor vec as u8
        let cursor_key_bytes = cursor.as_slice();
        has_previous_page = false;

        // reverse search : end_key → start_cursor have data
        if is_reverse_search {
            // start iteration form end_key
            let iter = db.iterator(IteratorMode::From(end_key.as_slice(), Direction::Reverse));
            // cursor is not "" : because cursor is null or "" (empty)
            if !cursor.is_empty() {
                for result in iter {
                    match result {
                        Ok((key, value)) => {
                            let key_ref = key.as_ref();

                            // current key is bigger than cursor key
                            // current key is not end_key : because start from end_key and might be current key and end_key is exact same key
                            if key_ref > cursor_key_bytes && key_ref != end_key.as_slice() {
                                let value_str = String::from_utf8_lossy(&value).to_string();
                                let json: serde_json::Value = serde_json::from_str(&value_str)?;

                                // filtering current data
                                if apply_filters(&json, &image_contains, &process_id_exact) {
                                    has_previous_page = true;
                                    break;
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Error from previousPage function (reverse search) : {},", e);
                            continue;
                        }
                    }
                }
            }
        } else {
            // forward search : start_key → start_cursor have data
            // start iteration form start_key
            let iter = db.iterator(IteratorMode::From(start_key.as_slice(), Direction::Forward));
            // cursor is not "" : because cursor is null or "" (empty)
            if !cursor.is_empty() {
                for result in iter {
                    match result {
                        Ok((key, value)) => {
                            let key_ref = key.as_ref();

                            // cursor key is bigger than current key
                            // current key is not start_key : because start from end_key and might be current key and start_key is exact same key
                            if key_ref < cursor_key_bytes && key_ref != start_key.as_slice() {
                                let value_str = String::from_utf8_lossy(&value).to_string();
                                let json: serde_json::Value = serde_json::from_str(&value_str)?;

                                // filtering current data
                                if apply_filters(&json, &image_contains, &process_id_exact) {
                                    has_previous_page = true;
                                    break;
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Error from previousPage function (forward search) : {},", e);
                            continue;
                        }
                    }
                }
            }
        }
    }

    // there is end_cursor string convert byte then convert as u8 array or empty array
    let end_cursor_bytes = end_cursor.as_ref().map(|s| s.as_bytes()).unwrap_or(&[]);

    // check for there is next data (page)
    if is_reverse_search {
        // reverse search : end_cursor → start_key have data
        let iter = db.iterator(IteratorMode::From(end_cursor_bytes, Direction::Reverse));
        for result in iter {
            match result {
                Ok((key, value)) => {
                    let key_ref = key.as_ref();

                    // current key is bigger than start_key
                    // current key is not end_cursor : because start from end_cursor and might be current key and end_cursor is exact same key
                    if key_ref > start_key.as_slice() && key_ref != end_cursor_bytes {
                        let value_str = String::from_utf8_lossy(&value).to_string();
                        let json: serde_json::Value = serde_json::from_str(&value_str)?;

                        // filtering current data
                        if apply_filters(&json, &image_contains, &process_id_exact) {
                            has_next_page = true;
                            break;
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error from nextPage function (reverse search) : {},", e);
                    continue;
                }
            }
        }
    } else {
        // Forward search: Iterate from start_key to cursor and check for data existence
        let iter = db.iterator(IteratorMode::From(end_cursor_bytes, Direction::Forward));
        for result in iter {
            match result {
                Ok((key, value)) => {
                    let key_ref = key.as_ref();

                    // end_key is bigger than current key
                    // current key is not end_cursor : because start from end_cursor and might be current key and end_cursor is exact same key
                    if key_ref < end_key.as_slice() && key_ref != end_cursor_bytes {
                        let value_str = String::from_utf8_lossy(&value).to_string();
                        let json: serde_json::Value = serde_json::from_str(&value_str)?;

                        // filtering current data
                        if apply_filters(&json, &image_contains, &process_id_exact) {
                            has_next_page = true;
                            break;
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error from nextPage function (forward search) : {},", e);
                    continue;
                }
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
    let image_contains = if let Some(ref image_contains_convert) = image_contains {
        // image_contain is empty sting "" then set as None
        if image_contains_convert.is_empty() {
            None
        // image_contain is have some stings then set as contained string
        } else {
            Some(image_contains_convert)
        }
    // image_contain is null then set as None
    } else {
        None
    };

    // process_id_exact is have some integer then set as contained integer
    let process_id_exact = if let Some(process_id_exact_convert) = process_id_exact {
        Some(*process_id_exact_convert)
    // process_id_exact is null then set as None
    } else {
        None
    };

    let image_condition = match image_contains {
        // if image_contains have strings
        // map with image with string contains image_contains_convert
        // if it's not contains goes false
        Some(image_contains_convert) => json["image"]
            .as_str()
            .map_or(false, |img| img.contains(image_contains_convert)),
        // image_contains is None than goes true
        None => true,
    };

    let process_id_condition = match process_id_exact {
        // if process_id_exact have integer
        // map with process_id with integer exact process_id_exact_convert
        // if it's not exact goes false
        Some(process_id_exact_convert) => json["process_id"]
            .as_u64()
            .map_or(false, |p| p == process_id_exact_convert as u64),
        // process_id_exact is None than goes true
        None => true,
    };

    image_condition && process_id_condition
}
