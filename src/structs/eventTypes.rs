extern crate chrono;
use chrono::{DateTime, Utc};
use std::net::IpAddr;
use serde::{Deserialize, Serialize};

// EVENT 1
#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessCreateEvent {
    pub agent_name: String,
    pub agent_id: String,
    pub event_action: String,
    pub utc_time: DateTime<Utc>,
    pub process_guid: String,
    pub process_id: u32,
    pub image: String,
    pub file_version: String,
    pub description: String,
    pub product: String,
    pub company: String,
    pub original_file_name: String,
    pub command_line: String,
    pub current_directory: String,
    pub user: String,
    pub logon_guid: String,
    pub logon_id: u32,
    pub terminal_session_id: u32,
    pub integrity_level: String,
    pub hashes: String,
    pub parent_process_guid: String,
    pub parent_process_id: u32,
    pub parent_image: String,
    pub parent_command_line: String,
    pub parent_user: String,
}

// EVENT 2
struct FileCreateTimeChangedEvent {
    agent_name: String,
    agent_id: String,
    event_action: String,
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
    agent_name: String,
    agent_id: String,
    event_action: String,
    utc_time: DateTime<Utc>,
    process_guid: String,
    process_id: u32,
    image: String,
    user: String,
    protocol: String,
    initiated: bool,
    source_is_ipv6: bool,
    source_ip: IpAddr,
    source_hostname: String,
    source_port: u16,
    source_port_name: String,
    destination_is_ipv6: bool,
    destination_ip: IpAddr,
    destination_hostname: String,
    destination_port: u16,
    destination_port_name: String,
}

// EVENT 4 : sysmon event 불필요
// struct SysmonServiceStateChangeEvent {
//     utc_time: DateTime<Utc>,
//     state: String,
//     version: String,
//     schema_version: String,
// }

// EVENT 5
struct ProcessTerminatedEvent {
    agent_name: String,
    agent_id: String,
    event_action: String,
    utc_time: DateTime<Utc>,
    process_guid: String,
    process_id: u32,
    image: String,
    user: String,
}

// EVENT 6
struct DriverLoadedEvent {
    agent_name: String,
    agent_id: String,
    event_action: String,
    utc_time: DateTime<Utc>,
    image_loaded: String,
    hashes: String,
    signed: bool,
    signature: String,
    signature_status: String,
}

// EVENT 7
struct ImageLoadedEvent {
    agent_name: String,
    agent_id: String,
    event_action: String,
    utc_time: DateTime<Utc>,
    process_guid: String,
    process_id: u32,
    image: String,
    image_loaded: String,
    file_version: String,
    description: String,
    product: String,
    company: String,
    original_file_name: String,
    hashes: String,
    signed: bool,
    signature: String,
    signature_status: String,
    user: String,
}

// EVENT 8
struct CreateRemoteThreadEvent {
    agent_name: String,
    agent_id: String,
    event_action: String,
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
    agent_name: String,
    agent_id: String,
    event_action: String,
    utc_time: DateTime<Utc>,
    process_guid: String,
    process_id: u32,
    image: String,
    device: String,
    user: String,
}

// EVENT 10
struct ProcessAccessedEvent {
    agent_name: String,
    agent_id: String,
    event_action: String,
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
    agent_name: String,
    agent_id: String,
    event_action: String,
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
    agent_name: String,
    agent_id: String,
    event_action: String,
    event_type: String,
    utc_time: DateTime<Utc>,
    process_guid: String,
    process_id: u32,
    image: String,
    target_object: String,
    user: String,
}

// EVENT 13
#[derive(Serialize, Deserialize, Debug)]
pub struct RegistryValueSetEvent {
    pub agent_name: String,
    pub agent_id: String,
    pub event_action: String,
    pub event_type: String,
    pub utc_time: DateTime<Utc>,
    pub process_guid: String,
    pub process_id: u32,
    pub image: String,
    pub target_object: String,
    pub details: String,
    pub user: String,
}

// EVENT 14
struct RegistryObjectRenamedEvent {
    agent_name: String,
    agent_id: String,
    event_action: String,
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
    agent_name: String,
    agent_id: String,
    event_action: String,
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

// EVENT 16 : sysmon event 불필요
// struct SysmonConfigStateChangedEvent {
//     utc_time: DateTime<Utc>,
//     configuration: String,
//     configuration_file_hash: String,
// }

// EVENT 17
struct PipeCreatedEvent {
    agent_name: String,
    agent_id: String,
    event_action: String,
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
    agent_name: String,
    agent_id: String,
    event_action: String,
    event_type: String,
    utc_time: DateTime<Utc>,
    process_guid: String,
    process_id: u32,
    pipe_name: String,
    image: String,
    user: String,
}

// Event 19, 20, 21 이벤트 미생성

// EVENT 22
struct DnsQueryEvent {
    agent_name: String,
    agent_id: String,
    event_action: String,
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
    agent_name: String,
    agent_id: String,
    event_action: String,
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
    agent_name: String,
    agent_id: String,
    event_action: String,
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
    agent_name: String,
    agent_id: String,
    event_action: String,
    utc_time: DateTime<Utc>,
    process_guid: String,
    process_id: u32,
    image: String,
    type_description: String,
    user: String,
}

// EVENT 26
struct FileDeleteLoggedEvent {
    agent_name: String,
    agent_id: String,
    event_action: String,
    utc_time: DateTime<Utc>,
    process_guid: String,
    process_id: u32,
    user: String,
    image: String,
    target_filename: String,
    hashes: String,
    is_executable: bool,
}
