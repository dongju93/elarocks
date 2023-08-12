use serde::Serialize;

// Sysmon structs with each evnet.code
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
    pub initiated: Option<String>,
    pub protocol: Option<String>,
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

#[derive(Serialize)]
pub struct Event13 {
    pub agent_name: Option<String>,
    pub agent_id: Option<String>,
    pub event_action: Option<String>,
    pub event_type: Option<String>,
    pub utc_time: Option<String>,
    pub process_guid: Option<String>,
    pub process_id: Option<String>,
    pub image: Option<String>,
    pub target_object: Option<String>,
    pub details: Option<String>,
    pub user: Option<String>,
}

#[derive(Serialize)]
pub struct Event14 {
    pub agent_name: Option<String>,
    pub agent_id: Option<String>,
    pub event_action: Option<String>,
    pub event_type: Option<String>,
    pub utc_time: Option<String>,
    pub process_guid: Option<String>,
    pub process_id: Option<String>,
    pub image: Option<String>,
    pub target_object: Option<String>,
    pub new_name: Option<String>,
    pub user: Option<String>,
}

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

#[derive(Serialize)]
pub struct Event17 {
    pub agent_name: Option<String>,
    pub agent_id: Option<String>,
    pub event_action: Option<String>,
    pub event_type: Option<String>,
    pub utc_time: Option<String>,
    pub process_guid: Option<String>,
    pub process_id: Option<String>,
    pub pipe_name: Option<String>,
    pub image: Option<String>,
    pub user: Option<String>,
}

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
