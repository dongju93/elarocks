mod implement;

use implement::EventToCSV;
use serde::Serialize;

// Sysmon structs with each evnet.code
// Process Create
#[derive(Serialize)]
pub struct Event1 {
    pub agent_name: Option<String>,
    pub agent_id: Option<String>,
    pub event_action: Option<String>,
    pub utc_time: Option<String>,
    pub process_guid: Option<String>,
    pub process_id: Option<String>,
    pub image: Option<String>,
    pub file_version: Option<String>,
    pub description: Option<String>,
    pub product: Option<String>,
    pub company: Option<String>,
    pub original_file_name: Option<String>,
    pub command_line: Option<String>,
    pub current_directory: Option<String>,
    pub user: Option<String>,
    pub logon_guid: Option<String>,
    pub logon_id: Option<String>,
    pub terminal_session_id: Option<String>,
    pub integrity_level: Option<String>,
    pub hashes: Option<String>,
    pub parent_process_guid: Option<String>,
    pub parent_process_id: Option<String>,
    pub parent_image: Option<String>,
    pub parent_command_line: Option<String>,
    pub parent_user: Option<String>,
}

// File creation time changed
#[derive(Serialize)]
pub struct Event2 {
    pub agent_name: Option<String>,
    pub agent_id: Option<String>,
    pub event_action: Option<String>,
    pub utc_time: Option<String>,
    pub process_guid: Option<String>,
    pub process_id: Option<String>,
    pub image: Option<String>,
    pub target_filename: Option<String>,
    pub creation_utc_time: Option<String>,
    pub previous_creation_utc_time: Option<String>,
    pub user: Option<String>,
}

// Network connection detected
#[derive(Serialize)]
pub struct Event3 {
    pub agent_name: Option<String>,
    pub agent_id: Option<String>,
    pub event_action: Option<String>,
    pub utc_time: Option<String>,
    pub process_guid: Option<String>,
    pub process_id: Option<String>,
    pub image: Option<String>,
    pub user: Option<String>,
    pub protocol: Option<String>,
    pub initiated: Option<String>,
    pub source_is_ipv6: Option<String>,
    pub source_ip: Option<String>,
    pub source_hostname: Option<String>,
    pub source_port: Option<String>,
    pub source_port_name: Option<String>,
    pub destination_is_ipv6: Option<String>,
    pub destination_ip: Option<String>,
    pub destination_hostname: Option<String>,
    pub destination_port: Option<String>,
    pub destination_port_name: Option<String>,
}

// Event4 : Sysmon service state changed 이벤트로 추출 불필요

// Process terminated
#[derive(Serialize)]
pub struct Event5 {
    pub agent_name: Option<String>,
    pub agent_id: Option<String>,
    pub event_action: Option<String>,
    pub utc_time: Option<String>,
    pub process_guid: Option<String>,
    pub process_id: Option<String>,
    pub image: Option<String>,
    pub user: Option<String>,
}

// Driver loaded
#[derive(Serialize)]
pub struct Event6 {
    pub agent_name: Option<String>,
    pub agent_id: Option<String>,
    pub event_action: Option<String>,
    pub utc_time: Option<String>,
    pub hashes: Option<String>,
    pub signed: Option<String>,
    pub signature: Option<String>,
    pub signature_status: Option<String>,
}

// Image loaded
#[derive(Serialize)]
pub struct Event7 {
    pub agent_name: Option<String>,
    pub agent_id: Option<String>,
    pub event_action: Option<String>,
    pub utc_time: Option<String>,
    pub process_guid: Option<String>,
    pub process_id: Option<String>,
    pub image: Option<String>,
    pub image_loaded: Option<String>,
    pub file_version: Option<String>,
    pub description: Option<String>,
    pub product: Option<String>,
    pub company: Option<String>,
    pub original_file_name: Option<String>,
    pub hashes: Option<String>,
    pub signed: Option<String>,
    pub signature: Option<String>,
    pub signature_status: Option<String>,
    pub user: Option<String>,
}

// CreateRemoteThread detected
#[derive(Serialize)]
pub struct Event8 {
    pub agent_name: Option<String>,
    pub agent_id: Option<String>,
    pub event_action: Option<String>,
    pub utc_time: Option<String>,
    pub source_process_guid: Option<String>,
    pub source_process_id: Option<String>,
    pub source_image: Option<String>,
    pub target_image: Option<String>,
    pub new_thread_id: Option<String>,
    pub start_address: Option<String>,
    pub start_module: Option<String>,
    pub start_function: Option<String>,
    pub source_user: Option<String>,
    pub target_user: Option<String>,
}

// RawAccessRead detected
#[derive(Serialize)]
pub struct Event9 {
    pub agent_name: Option<String>,
    pub agent_id: Option<String>,
    pub event_action: Option<String>,
    pub utc_time: Option<String>,
    pub process_guid: Option<String>,
    pub process_id: Option<String>,
    pub image: Option<String>,
    pub device: Option<String>,
    pub user: Option<String>,
}

// Process accessed
#[derive(Serialize)]
pub struct Event10 {
    pub agent_name: Option<String>,
    pub agent_id: Option<String>,
    pub event_action: Option<String>,
    pub utc_time: Option<String>,
    pub source_process_guid: Option<String>,
    pub source_process_id: Option<String>,
    pub source_thread_id: Option<String>,
    pub source_image: Option<String>,
    pub target_process_guid: Option<String>,
    pub target_process_id: Option<String>,
    pub target_thread_id: Option<String>,
    pub target_image: Option<String>,
    pub granted_access: Option<String>,
    pub call_trace: Option<String>,
    pub source_user: Option<String>,
    pub target_user: Option<String>,
}

// File created
#[derive(Serialize)]
pub struct Event11 {
    pub agent_name: Option<String>,
    pub agent_id: Option<String>,
    pub event_action: Option<String>,
    pub utc_time: Option<String>,
    pub process_guid: Option<String>,
    pub process_id: Option<String>,
    pub image: Option<String>,
    pub target_filename: Option<String>,
    pub creation_utc_time: Option<String>,
    pub user: Option<String>,
}

// Registry object added or deleted
#[derive(Serialize)]
pub struct Event12 {
    pub agent_name: Option<String>,
    pub agent_id: Option<String>,
    pub event_action: Option<String>,
    pub utc_time: Option<String>,
    pub event_type: Option<String>,
    pub process_guid: Option<String>,
    pub process_id: Option<String>,
    pub image: Option<String>,
    pub target_object: Option<String>,
    pub user: Option<String>,
}

// Registry value set
#[derive(Serialize)]
pub struct Event13 {
    pub agent_name: Option<String>,
    pub agent_id: Option<String>,
    pub event_action: Option<String>,
    pub utc_time: Option<String>,
    pub event_type: Option<String>,
    pub process_guid: Option<String>,
    pub process_id: Option<String>,
    pub image: Option<String>,
    pub target_object: Option<String>,
    pub details: Option<String>,
    pub user: Option<String>,
}

// Registry object renamed
#[derive(Serialize)]
pub struct Event14 {
    pub agent_name: Option<String>,
    pub agent_id: Option<String>,
    pub event_action: Option<String>,
    pub utc_time: Option<String>,
    pub event_type: Option<String>,
    pub process_guid: Option<String>,
    pub process_id: Option<String>,
    pub image: Option<String>,
    pub target_object: Option<String>,
    pub new_name: Option<String>,
    pub user: Option<String>,
}

// File stream created
#[derive(Serialize)]
pub struct Event15 {
    pub agent_name: Option<String>,
    pub agent_id: Option<String>,
    pub event_action: Option<String>,
    pub utc_time: Option<String>,
    pub process_guid: Option<String>,
    pub process_id: Option<String>,
    pub image: Option<String>,
    pub target_filename: Option<String>,
    pub creation_utc_time: Option<String>,
    pub hash: Option<String>,
    pub contents: Option<String>,
    pub user: Option<String>,
}

// Event16 : Sysmon config state changed 이벤트로 추출 불필요

// Pipe Created
#[derive(Serialize)]
pub struct Event17 {
    pub agent_name: Option<String>,
    pub agent_id: Option<String>,
    pub event_action: Option<String>,
    pub utc_time: Option<String>,
    pub event_type: Option<String>,
    pub process_guid: Option<String>,
    pub process_id: Option<String>,
    pub pipe_name: Option<String>,
    pub image: Option<String>,
    pub user: Option<String>,
}

// Pipe Connected
#[derive(Serialize)]
pub struct Event18 {
    pub agent_name: Option<String>,
    pub agent_id: Option<String>,
    pub event_action: Option<String>,
    pub utc_time: Option<String>,
    pub event_type: Option<String>,
    pub process_guid: Option<String>,
    pub process_id: Option<String>,
    pub pipe_name: Option<String>,
    pub image: Option<String>,
    pub user: Option<String>,
}

// Event 19, 20, 21 이벤트 미생성

// Dns query
#[derive(Serialize)]
pub struct Event22 {
    pub agent_name: Option<String>,
    pub agent_id: Option<String>,
    pub event_action: Option<String>,
    pub utc_time: Option<String>,
    pub process_guid: Option<String>,
    pub process_id: Option<String>,
    pub query_name: Option<String>,
    pub query_status: Option<String>,
    pub query_results: Option<String>,
    pub image: Option<String>,
    pub user: Option<String>,
}

// File Delete archived
#[derive(Serialize)]
pub struct Event23 {
    pub agent_name: Option<String>,
    pub agent_id: Option<String>,
    pub event_action: Option<String>,
    pub utc_time: Option<String>,
    pub process_guid: Option<String>,
    pub process_id: Option<String>,
    pub user: Option<String>,
    pub image: Option<String>,
    pub target_filename: Option<String>,
    pub hashes: Option<String>,
    pub is_executable: Option<String>,
    pub archived: Option<String>,
}

// Clipboard changed
#[derive(Serialize)]
pub struct Event24 {
    pub agent_name: Option<String>,
    pub agent_id: Option<String>,
    pub event_action: Option<String>,
    pub utc_time: Option<String>,
    pub process_guid: Option<String>,
    pub process_id: Option<String>,
    pub image: Option<String>,
    pub session: Option<String>,
    pub client_info: Option<String>,
    pub hashes: Option<String>,
    pub archived: Option<String>,
    pub user: Option<String>,
}

// Process Tampering
#[derive(Serialize)]
pub struct Event25 {
    pub agent_name: Option<String>,
    pub agent_id: Option<String>,
    pub event_action: Option<String>,
    pub utc_time: Option<String>,
    pub process_guid: Option<String>,
    pub process_id: Option<String>,
    pub image: Option<String>,
    pub types: Option<String>,
    pub user: Option<String>,
}

// File Delete logged
#[derive(Serialize)]
pub struct Event26 {
    pub agent_name: Option<String>,
    pub agent_id: Option<String>,
    pub event_action: Option<String>,
    pub utc_time: Option<String>,
    pub process_guid: Option<String>,
    pub process_id: Option<String>,
    pub user: Option<String>,
    pub image: Option<String>,
    pub target_filename: Option<String>,
    pub hashes: Option<String>,
    pub is_executable: Option<String>,
}

// Printout counts each events
pub(crate) fn process_event_data<T: EventToCSV>(
    data: &serde_json::Value,
    filename: &str,
    size: usize,
) {
    let entries = T::parse(data);
    println!("Data counts(Max: {size}): {}", entries.len());
    if let Err(e) = T::write_to_csv(&entries, filename) {
        eprintln!("Error writing to CSV: {e:?}");
    }
}
