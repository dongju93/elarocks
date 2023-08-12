Elasticsearch data to .csv file
===

collect SYSMON data with WINLOGBEAT and stored with ELASTICSEARCH and extract data to CSV files with delimiter "\t"

it's parsing "message" field with "agent.name", "agent.id" field

may required maximum size of query size, default size is 10000
```
// replace with your Index name
PUT /.ds-winlogbeat-8.8.2-2023.08.06-000001/_settings
{
    "max_result_window": 1000000
}
```

Please refer to the footnote in the code for detailed explanation

#### Copyright 2023. ClumL Inc. all rights reserved 