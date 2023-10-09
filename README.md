Elasticsearch data to .csv file
===

create config.toml file on root dir.
```
// Please replace "YOUR..." to your environments
es_url = "YOUR ELASTICSEARCH URL"
es_id = "YOUR ELASTICSEARCH ID"
es_pw = "YOUR ELASTICSEARCH PASSWORD"

// You can set multiple winlogbeat indices with array
indices = ["YOUR INDICES"]

// Adjust start and end time
start_time = "2023-08-25T15:00:00.000Z"
end_time = "2023-09-05T15:00:00.000Z"

// Adjust export size of data 
query_size = 100

// Csv file save location
save_location = "YOUR LOCATION"
```

You may need change maximum query size of elasticsearch first
```
// replace with your Index name
PUT /.ds-winlogbeat-8.8.2-2023.08.06-000001/_settings
{
    "max_result_window": 1000000
}
```
Build & Run
```
// build binary
cargo build --release

// run with config
./elarocks config.toml
```

(Option) To check dependencies size
```
// install bloat
cargo install cargo-bloat --no-default-features

// comment line out in Cargo.toml
#strip = true

// Check out
cargo bloat --release --crates
```
</br>

#### Copyright 2023. ClumL Inc. all rights reserved 