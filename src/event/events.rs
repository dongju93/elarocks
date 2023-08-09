use serde::Serialize;

#[derive(Serialize)]
pub struct EventOne {
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

#[derive(Serialize, Debug)]
pub struct EventTwo {
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
