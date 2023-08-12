Elasticsearch data to .csv file
===

First, you need collect [SYSMON](https://learn.microsoft.com/ko-kr/sysinternals/downloads/sysmon) data with [WINLOGBEAT](https://www.elastic.co/kr/beats/winlogbeat) and stored with [ELASTICSEARCH](https://www.elastic.co/kr/elasticsearch)   
Second, this code will extract data to CSV files with delimiter "\t"

it's parsing "message" field with "agent.name", "agent.id" field

may required modify maximum size of search query, default is 10000
```
// replace with your Index name
PUT /.ds-winlogbeat-8.8.2-2023.08.06-000001/_settings
{
    "max_result_window": 1000000
}
```

Please refer to the footnote in the code for detailed explanation

#### Copyright 2023. ClumL Inc. all rights reserved 