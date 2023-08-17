extern crate chrono;
use chrono::{DateTime, Utc};
use std::net::IpAddr;

// EVENT 1
struct ProcessCreateEvent {
    rule_name: Option<String>,
    utc_time: DateTime<Utc>,
    process_guid: String,
    process_id: u32,
    image: String,
    file_version: Option<String>,
    description: Option<String>,
    product: Option<String>,
    company: Option<String>,
    original_file_name: Option<String>,
    command_line: Option<String>,
    current_directory: Option<String>,
    user: String,
    logon_guid: Option<String>,
    logon_id: Option<u32>,
    terminal_session_id: Option<u32>,
    integrity_level: Option<String>,
    hashes: Option<String>,
    parent_process_guid: Option<String>,
    parent_process_id: Option<u32>,
    parent_image: Option<String>,
    parent_command_line: Option<String>,
    parent_user: Option<String>,
}

// EVENT 2
struct FileCreateTimeChangedEvent {
    rule_name: Option<String>,
    utc_time: DateTime<Utc>,
    process_guid: String,
    process_id: u32,
    image: String,
    target_filename: String,
    creation_utc_time: DateTime<Utc>,
    previous_creation_utc_time: DateTime<Utc>,
    user: String,
}

// EVENT 3
struct NetworkConnectionEvent {
    rule_name: Option<String>,
    utc_time: DateTime<Utc>,
    process_guid: String,
    process_id: u32,
    image: String,
    user: String,
    protocol: String,
    initiated: bool,
    source_is_ipv6: bool,
    source_ip: IpAddr,
    source_hostname: Option<String>,
    source_port: u16,
    source_port_name: Option<String>,
    destination_is_ipv6: bool,
    destination_ip: IpAddr,
    destination_hostname: Option<String>,
    destination_port: u16,
    destination_port_name: String,
}

// EVENT 4
struct SysmonServiceStateChangeEvent {
    utc_time: DateTime<Utc>,
    state: String,
    version: String,
    schema_version: String,
}

// EVENT 5
struct ProcessTerminatedEvent {
    rule_name: Option<String>,
    utc_time: DateTime<Utc>,
    process_guid: String,
    process_id: u32,
    image: String,
    user: String,
}

// EVENT 6
struct DriverLoadedEvent {
    rule_name: Option<String>,
    utc_time: DateTime<Utc>,
    image_loaded: String,
    hashes: String,
    signed: bool,
    signature: Option<String>,
    signature_status: Option<String>,
}

// EVENT 7
struct ImageLoadedEvent {
    rule_name: Option<String>,
    utc_time: DateTime<Utc>,
    process_guid: String,
    process_id: u32,
    image: String,
    image_loaded: String,
    file_version: Option<String>,
    description: Option<String>,
    product: Option<String>,
    company: Option<String>,
    original_file_name: Option<String>,
    hashes: String,
    signed: bool,
    signature: String,
    signature_status: String,
    user: String,
}

// EVENT 8
struct CreateRemoteThreadEvent {
    rule_name: Option<String>,
    utc_time: DateTime<Utc>,
    source_process_guid: String,
    source_process_id: u32,
    source_image: String,
    target_process_guid: String,
    target_process_id: u32,
    target_image: String,
    new_thread_id: u32,
    start_address: String,
    start_module: String,
    start_function: String,
    source_user: String,
    target_user: String,
}

// EVENT 9
struct RawAccessReadEvent {
    rule_name: Option<String>,
    utc_time: DateTime<Utc>,
    process_guid: String,
    process_id: u32,
    image: String,
    device: String,
    user: String,
}

// EVENT 10
struct ProcessAccessedEvent {
    rule_name: Option<String>,
    utc_time: DateTime<Utc>,
    source_process_guid: String,
    source_process_id: u32,
    source_thread_id: u32,
    source_image: String,
    target_process_guid: String,
    target_process_id: u32,
    target_image: String,
    granted_access: String,
    call_trace: String,
    source_user: String,
    target_user: String,
}

// EVENT 11
struct FileCreatedEvent {
    rule_name: Option<String>,
    utc_time: DateTime<Utc>,
    process_guid: String,
    process_id: u32,
    image: String,
    target_filename: String,
    creation_utc_time: DateTime<Utc>,
    user: String,
}

// EVENT 12
struct RegistryObjectAddedOrDeletedEvent {
    rule_name: Option<String>,
    event_type: String,
    utc_time: DateTime<Utc>,
    process_guid: String,
    process_id: u32,
    image: String,
    target_object: String,
    user: String,
}

// EVENT 13
struct RegistryValueSetEvent {
    rule_name: Option<String>,
    event_type: String,
    utc_time: DateTime<Utc>,
    process_guid: String,
    process_id: u32,
    image: String,
    target_object: String,
    details: String,
    user: String,
}

// EVENT 14
struct RegistryObjectRenamedEvent {
    rule_name: Option<String>,
    event_type: String,
    utc_time: DateTime<Utc>,
    process_guid: String,
    process_id: u32,
    image: String,
    target_object: String,
    new_name: String,
    user: String,
}

// EVENT 15
struct FileStreamCreatedEvent {
    rule_name: Option<String>,
    utc_time: DateTime<Utc>,
    process_guid: String,
    process_id: u32,
    image: String,
    target_filename: String,
    creation_utc_time: DateTime<Utc>,
    hash: String,
    contents: String,
    user: String,
}

// EVENT 16
struct SysmonConfigStateChangedEvent {
    utc_time: DateTime<Utc>,
    configuration: String,
    configuration_file_hash: String,
}

// EVENT 17
struct PipeCreatedEvent {
    rule_name: Option<String>,
    event_type: String,
    utc_time: DateTime<Utc>,
    process_guid: String,
    process_id: u32,
    pipe_name: String,
    image: String,
    user: String,
}

// EVENT 18
struct PipeConnectedEvent {
    rule_name: Option<String>,
    event_type: String,
    utc_time: DateTime<Utc>,
    process_guid: String,
    process_id: u32,
    pipe_name: String,
    image: String,
    user: String,
}

// EVENT 19
// EVENT 20
// EVENT 21
// 데이터 미생성

// EVENT 22
struct DnsQueryEvent {
    rule_name: Option<String>,
    utc_time: DateTime<Utc>,
    process_guid: String,
    process_id: u32,
    query_name: String,
    query_status: u32,
    query_results: String,
    image: String,
    user: String,
}

// EVENT 23
struct FileDeleteArchivedEvent {
    rule_name: Option<String>,
    utc_time: DateTime<Utc>,
    process_guid: String,
    process_id: u32,
    user: String,
    image: String,
    target_filename: String,
    hashes: String,
    is_executable: bool,
    archived: bool,
}

// EVENT 24
struct ClipboardChangedEvent {
    rule_name: Option<String>,
    utc_time: DateTime<Utc>,
    process_guid: String,
    process_id: u32,
    image: String,
    session: u32,
    client_info: String,
    hashes: String,
    archived: bool,
    user: String,
}

// EVENT 25
struct ProcessTamperingEvent {
    rule_name: Option<String>,
    utc_time: DateTime<Utc>,
    process_guid: String,
    process_id: u32,
    image: String,
    type_description: String,
    user: String,
}

// EVENT 26
struct FileDeleteLoggedEvent {
    rule_name: Option<String>,
    utc_time: DateTime<Utc>,
    process_guid: String,
    process_id: u32,
    user: String,
    image: String,
    target_filename: String,
    hashes: String,
    is_executable: bool,
}

// ... And so on for each event type ...
