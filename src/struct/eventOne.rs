pub struct EventOne {
    pub timestamp: Option<String>,
    pub event_type: Option<String>,
    pub rule_name: Option<String>,
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
    pub parent_user: Option<String>
}

pub fn create_event() -> EventOne {
    EventOne {
        timestamp: None,
        event_type: Some("Process Create".to_string()),
        rule_name: None,
        utc_time: None,
        process_guid: None,
        process_id: None,
        image: None,
        file_version: None,
        description: None,
        product: None,
        company: None,
        original_file_name: None,
        command_line: None,
        current_directory: None,
        user: None,
        logon_guid: None,
        logon_id: None,
        terminal_session_id: None,
        integrity_level: None,
        hashes: None,
        parent_process_guid: None,
        parent_process_id: None,
        parent_image: None,
        parent_command_line: None,
        parent_user: None
    }
}

impl EventOne {
    pub fn set_field(&mut self, key: &str, value: &str) {
        match key {
            "UtcTime" => entry.utc_time = Some(value.to_string()),
                            "ProcessGuid" => entry.process_guid = Some(value.to_string()),
                            "ProcessId" => entry.process_id = Some(value.to_string()),
                            "Image" => entry.image = Some(value.to_string()),
                            "FileVersion" => entry.file_version = Some(value.to_string()),
                            "Description" => entry.description = Some(value.to_string()),
                            "Product" => entry.product = Some(value.to_string()),
                            "Company" => entry.company = Some(value.to_string()),
                            "OriginalFileName" => entry.original_file_name = Some(value.to_string()),
                            "CommandLine" => entry.command_line = Some(value.to_string()),
                            "CurrentDirectory" => entry.current_directory = Some(value.to_string()),
                            "User" => entry.user = Some(value.to_string()),
                            "LogonGuid" => entry.logon_guid = Some(value.to_string()),
                            "LogonId" => entry.logon_id = Some(value.to_string()),
                            "TerminalSessionId" => entry.terminal_session_id = Some(value.to_string()),
                            "IntegrityLevel" => entry.integrity_level = Some(value.to_string()),
                            "Hashes" => entry.hashes = Some(value.to_string()),
                            "ParentProcessGuid" => entry.parent_process_guid = Some(value.to_string()),
                            "ParentProcessId" => entry.parent_process_id = Some(value.to_string()),
                            "ParentImage" => entry.parent_image = Some(value.to_string()),
                            "ParentCommandLine" => entry.parent_command_line = Some(value.to_string()),
                            "ParentUser" => entry.parent_user = Some(value.to_string()),
            _ => {}
        }
    }
}
