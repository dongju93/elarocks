Elasticsearch data to .csv file
===

create config.toml file on root dir.
```
// replace YOUR...
es_url = "YOUR ELASTICSEARCH URL"
es_id = "YOUR ELASTICSEARCH ID"
es_pw = "YOUR ELASTICSEARCH PASSWORD"

indices = ["YOUR INDICES"]

// ADJUST START AND END TIME
start_time = "2023-08-25T15:00:00.000Z"
end_time = "2023-09-05T15:00:00.000Z"

// ADJUST EXPORT SIZE OF DATA
query_size = 100

// CSV FILE LOCATIOS
save_location = "YOUR LOCATION"
```

Build & Run
```
// build binary
cargo build --release

// run with config
./elarocks config.toml
```

To check dependencies size
```
// install bloat
cargo install cargo-bloat --no-default-features

// comment line out in Cargo.toml
#strip = true

// Check out
cargo bloat --release --crates
//
```
</br>

#### Copyright 2023. ClumL Inc. all rights reserved 