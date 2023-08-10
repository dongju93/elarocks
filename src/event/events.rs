use serde::Serialize;

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

#[derive(Serialize)] // We're using the serde crate's Serialize trait to help with CSV writing
struct Event3 {
    agent_name: Option<String>,
    agent_id: Option<String>,
    event_action: Option<String>,
    utc_time: Option<String>,
    process_guid: Option<String>,
    process_id: Option<String>,
    image: Option<String>,
    user: Option<String>,
    initiated: Option<String>,
    protocol: Option<String>,
    source_is_ipv6: Option<String>,
    source_ip: Option<String>,
    source_hostname: Option<String>,
    source_port: Option<String>,
    source_port_name: Option<String>,
    destination_is_ipv6: Option<String>,
    destination_ip: Option<String>,
    destination_hostname: Option<String>,
    destination_port: Option<String>,
    destination_port_name: Option<String>,
}

#[derive(Serialize)] // We're using the serde crate's Serialize trait to help with CSV writing
struct Event5 {
    agent_name: Option<String>,
    agent_id: Option<String>,
    event_action: Option<String>,
    utc_time: Option<String>,
    process_guid: Option<String>,
    process_id: Option<String>,
    image: Option<String>,
    user: Option<String>,
}

#[derive(Serialize)] // We're using the serde crate's Serialize trait to help with CSV writing
struct Event7 {
    agent_name: Option<String>,
    agent_id: Option<String>,
    event_action: Option<String>,
    utc_time: Option<String>,
    process_guid: Option<String>,
    process_id: Option<String>,
    image: Option<String>,
    image_loaded: Option<String>,
    file_version: Option<String>,
    description: Option<String>,
    product: Option<String>,
    company: Option<String>,
    original_file_name: Option<String>,
    hashes: Option<String>,
    signed: Option<String>,
    signature: Option<String>,
    signature_status: Option<String>,
    user: Option<String>,
}

#[derive(Serialize)] // We're using the serde crate's Serialize trait to help with CSV writing
struct Event11 {
    agent_name: Option<String>,
    agent_id: Option<String>,
    event_action: Option<String>,
    utc_time: Option<String>,
    process_guid: Option<String>,
    process_id: Option<String>,
    image: Option<String>,
    target_filename: Option<String>,
    creation_utc_time: Option<String>,
    user: Option<String>,
}

#[derive(Serialize)] // We're using the serde crate's Serialize trait to help with CSV writing
struct Event13 {
    agent_name: Option<String>,
    agent_id: Option<String>,
    event_action: Option<String>,
    event_type: Option<String>,
    utc_time: Option<String>,
    process_guid: Option<String>,
    process_id: Option<String>,
    image: Option<String>,
    target_object: Option<String>,
    details: Option<String>,
    user: Option<String>,
}

