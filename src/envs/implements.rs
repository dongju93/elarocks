use crate::structs::events::*;
use std::{fs, io};

// A common interface for a group of types.
pub(crate) trait EventToCSV: Sized {
    fn parse(data: &serde_json::Value) -> Vec<Self>;
    fn write_to_csv(entries: &Vec<Self>, filename: &str) -> std::io::Result<()>;
}

// Parse json(response data) and make .csv files
impl EventToCSV for Event1 {
    fn parse(data: &serde_json::Value) -> Vec<Self> {
        let mut entries = Vec::new();

        if let Some(hits) = data["hits"]["hits"].as_array() {
            for hit in hits {
                if let Some(message) = hit["_source"]["message"].as_str() {
                    let mut entry = Event1 {
                        agent_name: None,
                        agent_id: None,
                        event_action: Some("Process Create".to_string()),
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
                        parent_user: None,
                    };

                    if let Some(agent_name) = hit["_source"]["agent"]["name"].as_str() {
                        entry.agent_name = Some(agent_name.to_string());
                    }

                    if let Some(agent_id) = hit["_source"]["agent"]["id"].as_str() {
                        entry.agent_id = Some(agent_id.to_string());
                    }

                    for part in message.split('\n') {
                        let segments: Vec<_> = part.splitn(2, ':').collect();
                        // println!("{:?}", segments); // Debug prints
                        if segments.len() == 2 {
                            let key = segments[0].trim();
                            let value = segments[1].trim();
                            match key {
                                "UtcTime" => entry.utc_time = Some(value.to_string()),
                                "ProcessGuid" => entry.process_guid = Some(value.to_string()),
                                "ProcessId" => entry.process_id = Some(value.to_string()),
                                "Image" => entry.image = Some(value.to_string()),
                                "FileVersion" => entry.file_version = Some(value.to_string()),
                                "Description" => entry.description = Some(value.to_string()),
                                "Product" => entry.product = Some(value.to_string()),
                                "Company" => entry.company = Some(value.to_string()),
                                "OriginalFileName" => {
                                    entry.original_file_name = Some(value.to_string())
                                }
                                "CommandLine" => entry.command_line = Some(value.to_string()),
                                "CurrentDirectory" => {
                                    entry.current_directory = Some(value.to_string())
                                }
                                "User" => entry.user = Some(value.to_string()),
                                "LogonGuid" => entry.logon_guid = Some(value.to_string()),
                                "LogonId" => entry.logon_id = Some(value.to_string()),
                                "TerminalSessionId" => {
                                    entry.terminal_session_id = Some(value.to_string())
                                }
                                "IntegrityLevel" => entry.integrity_level = Some(value.to_string()),
                                "Hashes" => entry.hashes = Some(value.to_string()),
                                "ParentProcessGuid" => {
                                    entry.parent_process_guid = Some(value.to_string())
                                }
                                "ParentProcessId" => {
                                    entry.parent_process_id = Some(value.to_string())
                                }
                                "ParentImage" => entry.parent_image = Some(value.to_string()),
                                "ParentCommandLine" => {
                                    entry.parent_command_line = Some(value.to_string())
                                }
                                "ParentUser" => entry.parent_user = Some(value.to_string()),
                                _ => {}
                            }
                        }
                    }

                    entries.push(entry);
                }
            }
        }

        entries
    }

    fn write_to_csv(entries: &Vec<Self>, filename: &str) -> io::Result<()> {
        // Check if entries is empty, and if so, return early.
        if entries.is_empty() {
            return Ok(());
        }

        let file_exists = fs::metadata(filename).is_ok();

        let mut wtr = if file_exists {
            // Open the file in append mode if it exists.
            csv::WriterBuilder::new()
                .delimiter(b'\t')
                .has_headers(false) // Don't write headers when appending.
                .from_writer(fs::OpenOptions::new().append(true).open(filename)?)
        } else {
            // Create a new file if it doesn't exist.
            csv::WriterBuilder::new()
                .delimiter(b'\t')
                .from_path(filename)?
        };

        for entry in entries {
            wtr.serialize(entry)?;
        }
        wtr.flush()?;
        Ok(())
    }
}

impl EventToCSV for Event2 {
    fn parse(data: &serde_json::Value) -> Vec<Self> {
        let mut entries = Vec::new();

        if let Some(hits) = data["hits"]["hits"].as_array() {
            for hit in hits {
                if let Some(message) = hit["_source"]["message"].as_str() {
                    // println!("Event2 raw message: {}", message);
                    let mut entry = Event2 {
                        agent_name: None,
                        agent_id: None,
                        event_action: Some("File creation time changed".to_string()),
                        utc_time: None,
                        process_guid: None,
                        process_id: None,
                        image: None,
                        target_filename: None,
                        creation_utc_time: None,
                        previous_creation_utc_time: None,
                        user: None,
                    };

                    if let Some(agent_name) = hit["_source"]["agent"]["name"].as_str() {
                        entry.agent_name = Some(agent_name.to_string());
                    }

                    if let Some(agent_id) = hit["_source"]["agent"]["id"].as_str() {
                        entry.agent_id = Some(agent_id.to_string());
                    }

                    for part in message.split('\n') {
                        let segments: Vec<_> = part.splitn(2, ':').collect();
                        // println!("{:?}", segments); // Debug prints
                        if segments.len() == 2 {
                            let key = segments[0].trim();
                            let value = segments[1].trim();
                            match key {
                                "UtcTime" => entry.utc_time = Some(value.to_string()),
                                "ProcessGuid" => entry.process_guid = Some(value.to_string()),
                                "ProcessId" => entry.process_id = Some(value.to_string()),
                                "Image" => entry.image = Some(value.to_string()),
                                "TargetFilename" => entry.target_filename = Some(value.to_string()),
                                "CreationUtcTime" => {
                                    entry.creation_utc_time = Some(value.to_string())
                                }
                                "PreviousCreationUtcTime" => {
                                    entry.previous_creation_utc_time = Some(value.to_string())
                                }
                                "User" => entry.user = Some(value.to_string()),
                                _ => {}
                            }
                        }
                    }

                    entries.push(entry);
                }
            }
        }

        entries
    }

    fn write_to_csv(entries: &Vec<Self>, filename: &str) -> io::Result<()> {
        // Check if entries is empty, and if so, return early.
        if entries.is_empty() {
            return Ok(());
        }

        let file_exists = fs::metadata(filename).is_ok();

        let mut wtr = if file_exists {
            // Open the file in append mode if it exists.
            csv::WriterBuilder::new()
                .delimiter(b'\t')
                .has_headers(false) // Don't write headers when appending.
                .from_writer(fs::OpenOptions::new().append(true).open(filename)?)
        } else {
            // Create a new file if it doesn't exist.
            csv::WriterBuilder::new()
                .delimiter(b'\t')
                .from_path(filename)?
        };

        for entry in entries {
            wtr.serialize(entry)?;
        }
        wtr.flush()?;
        Ok(())
    }
}

impl EventToCSV for Event3 {
    fn parse(data: &serde_json::Value) -> Vec<Self> {
        let mut entries = Vec::new();

        if let Some(hits) = data["hits"]["hits"].as_array() {
            for hit in hits {
                if let Some(message) = hit["_source"]["message"].as_str() {
                    // println!("Event3 raw message: {}", message);
                    let mut entry = Event3 {
                        agent_name: None,
                        agent_id: None,
                        event_action: Some("Network connection detected".to_string()),
                        utc_time: None,
                        process_guid: None,
                        process_id: None,
                        image: None,
                        user: None,
                        protocol: None,
                        initiated: None,
                        source_is_ipv6: None,
                        source_ip: None,
                        source_hostname: None,
                        source_port: None,
                        source_port_name: None,
                        destination_is_ipv6: None,
                        destination_ip: None,
                        destination_hostname: None,
                        destination_port: None,
                        destination_port_name: None,
                    };

                    if let Some(agent_name) = hit["_source"]["agent"]["name"].as_str() {
                        entry.agent_name = Some(agent_name.to_string());
                    }

                    if let Some(agent_id) = hit["_source"]["agent"]["id"].as_str() {
                        entry.agent_id = Some(agent_id.to_string());
                    }

                    for part in message.split('\n') {
                        let segments: Vec<_> = part.splitn(2, ':').collect();
                        // println!("{:?}", segments); // Debug prints
                        if segments.len() == 2 {
                            let key = segments[0].trim();
                            let value = segments[1].trim();
                            match key {
                                "UtcTime" => entry.utc_time = Some(value.to_string()),
                                "ProcessGuid" => entry.process_guid = Some(value.to_string()),
                                "ProcessId" => entry.process_id = Some(value.to_string()),
                                "Image" => entry.image = Some(value.to_string()),
                                "User" => entry.user = Some(value.to_string()),
                                "Protocol" => entry.protocol = Some(value.to_string()),
                                "Initiated" => entry.initiated = Some(value.to_string()),
                                "SourceIsIpv6" => entry.source_is_ipv6 = Some(value.to_string()),
                                "SourceIp" => entry.source_ip = Some(value.to_string()),
                                "SourceHostname" => entry.source_hostname = Some(value.to_string()),
                                "SourcePort" => entry.source_port = Some(value.to_string()),
                                "SourcePortName" => {
                                    entry.source_port_name = Some(value.to_string())
                                }
                                "DestinationIsIpv6" => {
                                    entry.destination_is_ipv6 = Some(value.to_string())
                                }
                                "DestinationIp" => entry.destination_ip = Some(value.to_string()),
                                "DestinationHostname" => {
                                    entry.destination_hostname = Some(value.to_string())
                                }
                                "DestinationPort" => {
                                    entry.destination_port = Some(value.to_string())
                                }
                                "DestinationPortName" => {
                                    entry.destination_port_name = Some(value.to_string())
                                }
                                _ => {}
                            }
                        }
                    }

                    entries.push(entry);
                }
            }
        }

        entries
    }

    fn write_to_csv(entries: &Vec<Self>, filename: &str) -> io::Result<()> {
        // Check if entries is empty, and if so, return early.
        if entries.is_empty() {
            return Ok(());
        }

        let file_exists = fs::metadata(filename).is_ok();

        let mut wtr = if file_exists {
            // Open the file in append mode if it exists.
            csv::WriterBuilder::new()
                .delimiter(b'\t')
                .has_headers(false) // Don't write headers when appending.
                .from_writer(fs::OpenOptions::new().append(true).open(filename)?)
        } else {
            // Create a new file if it doesn't exist.
            csv::WriterBuilder::new()
                .delimiter(b'\t')
                .from_path(filename)?
        };

        for entry in entries {
            wtr.serialize(entry)?;
        }
        wtr.flush()?;
        Ok(())
    }
}

impl EventToCSV for Event5 {
    fn parse(data: &serde_json::Value) -> Vec<Self> {
        let mut entries = Vec::new();

        if let Some(hits) = data["hits"]["hits"].as_array() {
            for hit in hits {
                if let Some(message) = hit["_source"]["message"].as_str() {
                    // println!("Event5 raw message: {}", message);
                    let mut entry = Event5 {
                        agent_name: None,
                        agent_id: None,
                        event_action: Some("Process terminated".to_string()),
                        utc_time: None,
                        process_guid: None,
                        process_id: None,
                        image: None,
                        user: None,
                    };

                    if let Some(agent_name) = hit["_source"]["agent"]["name"].as_str() {
                        entry.agent_name = Some(agent_name.to_string());
                    }

                    if let Some(agent_id) = hit["_source"]["agent"]["id"].as_str() {
                        entry.agent_id = Some(agent_id.to_string());
                    }

                    for part in message.split('\n') {
                        let segments: Vec<_> = part.splitn(2, ':').collect();
                        // println!("{:?}", segments); // Debug prints
                        if segments.len() == 2 {
                            let key = segments[0].trim();
                            let value = segments[1].trim();
                            match key {
                                "UtcTime" => entry.utc_time = Some(value.to_string()),
                                "ProcessGuid" => entry.process_guid = Some(value.to_string()),
                                "ProcessId" => entry.process_id = Some(value.to_string()),
                                "Image" => entry.image = Some(value.to_string()),
                                "User" => entry.user = Some(value.to_string()),
                                _ => {}
                            }
                        }
                    }

                    entries.push(entry);
                }
            }
        }

        entries
    }

    fn write_to_csv(entries: &Vec<Self>, filename: &str) -> io::Result<()> {
        // Check if entries is empty, and if so, return early.
        if entries.is_empty() {
            return Ok(());
        }

        let file_exists = fs::metadata(filename).is_ok();

        let mut wtr = if file_exists {
            // Open the file in append mode if it exists.
            csv::WriterBuilder::new()
                .delimiter(b'\t')
                .has_headers(false) // Don't write headers when appending.
                .from_writer(fs::OpenOptions::new().append(true).open(filename)?)
        } else {
            // Create a new file if it doesn't exist.
            csv::WriterBuilder::new()
                .delimiter(b'\t')
                .from_path(filename)?
        };

        for entry in entries {
            wtr.serialize(entry)?;
        }
        wtr.flush()?;
        Ok(())
    }
}

impl EventToCSV for Event7 {
    fn parse(data: &serde_json::Value) -> Vec<Self> {
        let mut entries = Vec::new();

        if let Some(hits) = data["hits"]["hits"].as_array() {
            for hit in hits {
                if let Some(message) = hit["_source"]["message"].as_str() {
                    // println!("Event7 raw message: {}", message);
                    let mut entry = Event7 {
                        agent_name: None,
                        agent_id: None,
                        event_action: Some("Image loaded".to_string()),
                        utc_time: None,
                        process_guid: None,
                        process_id: None,
                        image: None,
                        image_loaded: None,
                        file_version: None,
                        description: None,
                        product: None,
                        company: None,
                        original_file_name: None,
                        hashes: None,
                        signed: None,
                        signature: None,
                        signature_status: None,
                        user: None,
                    };

                    if let Some(agent_name) = hit["_source"]["agent"]["name"].as_str() {
                        entry.agent_name = Some(agent_name.to_string());
                    }

                    if let Some(agent_id) = hit["_source"]["agent"]["id"].as_str() {
                        entry.agent_id = Some(agent_id.to_string());
                    }

                    for part in message.split('\n') {
                        let segments: Vec<_> = part.splitn(2, ':').collect();
                        // println!("{:?}", segments); // Debug prints
                        if segments.len() == 2 {
                            let key = segments[0].trim();
                            let value = segments[1].trim();
                            match key {
                                "UtcTime" => entry.utc_time = Some(value.to_string()),
                                "ProcessGuid" => entry.process_guid = Some(value.to_string()),
                                "ProcessId" => entry.process_id = Some(value.to_string()),
                                "Image" => entry.image = Some(value.to_string()),
                                "ImageLoaded" => entry.image_loaded = Some(value.to_string()),
                                "FileVersion" => entry.file_version = Some(value.to_string()),
                                "Description" => entry.description = Some(value.to_string()),
                                "Product" => entry.product = Some(value.to_string()),
                                "Company" => entry.company = Some(value.to_string()),
                                "OriginalFileName" => {
                                    entry.original_file_name = Some(value.to_string())
                                }
                                "Hashes" => entry.hashes = Some(value.to_string()),
                                "Signed" => entry.signed = Some(value.to_string()),
                                "Signature" => entry.signature = Some(value.to_string()),
                                "SignatureStatus" => {
                                    entry.signature_status = Some(value.to_string())
                                }
                                "User" => entry.user = Some(value.to_string()),
                                _ => {}
                            }
                        }
                    }

                    entries.push(entry);
                }
            }
        }

        entries
    }

    fn write_to_csv(entries: &Vec<Self>, filename: &str) -> io::Result<()> {
        // Check if entries is empty, and if so, return early.
        if entries.is_empty() {
            return Ok(());
        }

        let file_exists = fs::metadata(filename).is_ok();

        let mut wtr = if file_exists {
            // Open the file in append mode if it exists.
            csv::WriterBuilder::new()
                .delimiter(b'\t')
                .has_headers(false) // Don't write headers when appending.
                .from_writer(fs::OpenOptions::new().append(true).open(filename)?)
        } else {
            // Create a new file if it doesn't exist.
            csv::WriterBuilder::new()
                .delimiter(b'\t')
                .from_path(filename)?
        };

        for entry in entries {
            wtr.serialize(entry)?;
        }
        wtr.flush()?;
        Ok(())
    }
}

impl EventToCSV for Event9 {
    fn parse(data: &serde_json::Value) -> Vec<Self> {
        let mut entries = Vec::new();

        if let Some(hits) = data["hits"]["hits"].as_array() {
            for hit in hits {
                if let Some(message) = hit["_source"]["message"].as_str() {
                    // println!("Event9 raw message: {}", message);
                    let mut entry = Event9 {
                        agent_name: None,
                        agent_id: None,
                        event_action: Some("RawAccessRead detected".to_string()),
                        utc_time: None,
                        process_guid: None,
                        process_id: None,
                        image: None,
                        device: None,
                        user: None,
                    };

                    if let Some(agent_name) = hit["_source"]["agent"]["name"].as_str() {
                        entry.agent_name = Some(agent_name.to_string());
                    }

                    if let Some(agent_id) = hit["_source"]["agent"]["id"].as_str() {
                        entry.agent_id = Some(agent_id.to_string());
                    }

                    for part in message.split('\n') {
                        let segments: Vec<_> = part.splitn(2, ':').collect();
                        // println!("{:?}", segments); // Debug prints
                        if segments.len() == 2 {
                            let key = segments[0].trim();
                            let value = segments[1].trim();
                            match key {
                                "UtcTime" => entry.utc_time = Some(value.to_string()),
                                "ProcessGuid" => entry.process_guid = Some(value.to_string()),
                                "ProcessId" => entry.process_id = Some(value.to_string()),
                                "Image" => entry.image = Some(value.to_string()),
                                "Device" => entry.device = Some(value.to_string()),
                                "User" => entry.user = Some(value.to_string()),
                                _ => {}
                            }
                        }
                    }

                    entries.push(entry);
                }
            }
        }

        entries
    }

    fn write_to_csv(entries: &Vec<Self>, filename: &str) -> io::Result<()> {
        // Check if entries is empty, and if so, return early.
        if entries.is_empty() {
            return Ok(());
        }

        let file_exists = fs::metadata(filename).is_ok();

        let mut wtr = if file_exists {
            // Open the file in append mode if it exists.
            csv::WriterBuilder::new()
                .delimiter(b'\t')
                .has_headers(false) // Don't write headers when appending.
                .from_writer(fs::OpenOptions::new().append(true).open(filename)?)
        } else {
            // Create a new file if it doesn't exist.
            csv::WriterBuilder::new()
                .delimiter(b'\t')
                .from_path(filename)?
        };

        for entry in entries {
            wtr.serialize(entry)?;
        }
        wtr.flush()?;
        Ok(())
    }
}

impl EventToCSV for Event10 {
    fn parse(data: &serde_json::Value) -> Vec<Self> {
        let mut entries = Vec::new();

        if let Some(hits) = data["hits"]["hits"].as_array() {
            for hit in hits {
                if let Some(message) = hit["_source"]["message"].as_str() {
                    // println!("Event10 raw message: {}", message);
                    let mut entry = Event10 {
                        agent_name: None,
                        agent_id: None,
                        event_action: Some("Process accessed".to_string()),
                        utc_time: None,
                        source_process_guid: None,
                        source_process_id: None,
                        source_thread_id: None,
                        source_image: None,
                        target_process_guid: None,
                        target_process_id: None,
                        target_thread_id: None,
                        target_image: None,
                        granted_access: None,
                        call_trace: None,
                        source_user: None,
                        target_user: None,
                    };

                    if let Some(agent_name) = hit["_source"]["agent"]["name"].as_str() {
                        entry.agent_name = Some(agent_name.to_string());
                    }

                    if let Some(agent_id) = hit["_source"]["agent"]["id"].as_str() {
                        entry.agent_id = Some(agent_id.to_string());
                    }

                    for part in message.split('\n') {
                        let segments: Vec<_> = part.splitn(2, ':').collect();
                        // println!("{:?}", segments); // Debug prints
                        if segments.len() == 2 {
                            let key = segments[0].trim();
                            let value = segments[1].trim();
                            match key {
                                "UtcTime" => entry.utc_time = Some(value.to_string()),
                                "SourceProcessGuid" => {
                                    entry.source_process_guid = Some(value.to_string())
                                }
                                "SourceProcessId" => {
                                    entry.source_process_id = Some(value.to_string())
                                }
                                "SourceThreadId" => {
                                    entry.source_thread_id = Some(value.to_string())
                                }
                                "SourceImage" => entry.source_image = Some(value.to_string()),
                                "TargetProcessGuid" => {
                                    entry.target_process_guid = Some(value.to_string())
                                }
                                "TargetProcessId" => {
                                    entry.target_process_id = Some(value.to_string())
                                }
                                "TargetThreadId" => {
                                    entry.target_thread_id = Some(value.to_string())
                                }
                                "TargetImage" => entry.target_image = Some(value.to_string()),
                                "GrantedAccess" => entry.granted_access = Some(value.to_string()),
                                "CallTrace" => entry.call_trace = Some(value.to_string()),
                                "SourceUser" => entry.source_user = Some(value.to_string()),
                                "TargetUser" => entry.target_user = Some(value.to_string()),
                                _ => {}
                            }
                        }
                    }

                    entries.push(entry);
                }
            }
        }

        entries
    }

    fn write_to_csv(entries: &Vec<Self>, filename: &str) -> io::Result<()> {
        // Check if entries is empty, and if so, return early.
        if entries.is_empty() {
            return Ok(());
        }

        let file_exists = fs::metadata(filename).is_ok();

        let mut wtr = if file_exists {
            // Open the file in append mode if it exists.
            csv::WriterBuilder::new()
                .delimiter(b'\t')
                .has_headers(false) // Don't write headers when appending.
                .from_writer(fs::OpenOptions::new().append(true).open(filename)?)
        } else {
            // Create a new file if it doesn't exist.
            csv::WriterBuilder::new()
                .delimiter(b'\t')
                .from_path(filename)?
        };

        for entry in entries {
            wtr.serialize(entry)?;
        }
        wtr.flush()?;
        Ok(())
    }
}

impl EventToCSV for Event11 {
    fn parse(data: &serde_json::Value) -> Vec<Self> {
        let mut entries = Vec::new();

        if let Some(hits) = data["hits"]["hits"].as_array() {
            for hit in hits {
                if let Some(message) = hit["_source"]["message"].as_str() {
                    // println!("Event11 raw message: {}", message);
                    let mut entry = Event11 {
                        agent_name: None,
                        agent_id: None,
                        event_action: Some("File created".to_string()),
                        utc_time: None,
                        process_guid: None,
                        process_id: None,
                        image: None,
                        target_filename: None,
                        creation_utc_time: None,
                        user: None,
                    };

                    if let Some(agent_name) = hit["_source"]["agent"]["name"].as_str() {
                        entry.agent_name = Some(agent_name.to_string());
                    }

                    if let Some(agent_id) = hit["_source"]["agent"]["id"].as_str() {
                        entry.agent_id = Some(agent_id.to_string());
                    }

                    for part in message.split('\n') {
                        let segments: Vec<_> = part.splitn(2, ':').collect();
                        // println!("{:?}", segments); // Debug prints
                        if segments.len() == 2 {
                            let key = segments[0].trim();
                            let value = segments[1].trim();
                            match key {
                                "UtcTime" => entry.utc_time = Some(value.to_string()),
                                "ProcessGuid" => entry.process_guid = Some(value.to_string()),
                                "ProcessId" => entry.process_id = Some(value.to_string()),
                                "Image" => entry.image = Some(value.to_string()),
                                "TargetFilename" => entry.target_filename = Some(value.to_string()),
                                "CreationUtcTime" => {
                                    entry.creation_utc_time = Some(value.to_string())
                                }
                                "User" => entry.user = Some(value.to_string()),
                                _ => {}
                            }
                        }
                    }

                    entries.push(entry);
                }
            }
        }

        entries
    }

    fn write_to_csv(entries: &Vec<Self>, filename: &str) -> io::Result<()> {
        // Check if entries is empty, and if so, return early.
        if entries.is_empty() {
            return Ok(());
        }

        let file_exists = fs::metadata(filename).is_ok();

        let mut wtr = if file_exists {
            // Open the file in append mode if it exists.
            csv::WriterBuilder::new()
                .delimiter(b'\t')
                .has_headers(false) // Don't write headers when appending.
                .from_writer(fs::OpenOptions::new().append(true).open(filename)?)
        } else {
            // Create a new file if it doesn't exist.
            csv::WriterBuilder::new()
                .delimiter(b'\t')
                .from_path(filename)?
        };

        for entry in entries {
            wtr.serialize(entry)?;
        }
        wtr.flush()?;
        Ok(())
    }
}

impl EventToCSV for Event12 {
    fn parse(data: &serde_json::Value) -> Vec<Self> {
        let mut entries = Vec::new();

        if let Some(hits) = data["hits"]["hits"].as_array() {
            for hit in hits {
                if let Some(message) = hit["_source"]["message"].as_str() {
                    // println!("Event12 raw message: {}", message);
                    let mut entry = Event12 {
                        agent_name: None,
                        agent_id: None,
                        event_action: Some("Registry object added or deleted".to_string()),
                        utc_time: None,
                        event_type: None,
                        process_guid: None,
                        process_id: None,
                        image: None,
                        target_object: None,
                        user: None,
                    };

                    if let Some(agent_name) = hit["_source"]["agent"]["name"].as_str() {
                        entry.agent_name = Some(agent_name.to_string());
                    }

                    if let Some(agent_id) = hit["_source"]["agent"]["id"].as_str() {
                        entry.agent_id = Some(agent_id.to_string());
                    }

                    for part in message.split('\n') {
                        let segments: Vec<_> = part.splitn(2, ':').collect();
                        // println!("{:?}", segments); // Debug prints
                        if segments.len() == 2 {
                            let key = segments[0].trim();
                            let value = segments[1].trim();
                            match key {
                                "UtcTime" => entry.utc_time = Some(value.to_string()),
                                "EventType" => entry.event_type = Some(value.to_string()),
                                "ProcessGuid" => entry.process_guid = Some(value.to_string()),
                                "ProcessId" => entry.process_id = Some(value.to_string()),
                                "Image" => entry.image = Some(value.to_string()),
                                "TargetObject" => entry.target_object = Some(value.to_string()),
                                "User" => entry.user = Some(value.to_string()),
                                _ => {}
                            }
                        }
                    }

                    entries.push(entry);
                }
            }
        }

        entries
    }

    fn write_to_csv(entries: &Vec<Self>, filename: &str) -> io::Result<()> {
        // Check if entries is empty, and if so, return early.
        if entries.is_empty() {
            return Ok(());
        }

        let file_exists = fs::metadata(filename).is_ok();

        let mut wtr = if file_exists {
            // Open the file in append mode if it exists.
            csv::WriterBuilder::new()
                .delimiter(b'\t')
                .has_headers(false) // Don't write headers when appending.
                .from_writer(fs::OpenOptions::new().append(true).open(filename)?)
        } else {
            // Create a new file if it doesn't exist.
            csv::WriterBuilder::new()
                .delimiter(b'\t')
                .from_path(filename)?
        };

        for entry in entries {
            wtr.serialize(entry)?;
        }
        wtr.flush()?;
        Ok(())
    }
}

impl EventToCSV for Event13 {
    fn parse(data: &serde_json::Value) -> Vec<Self> {
        let mut entries = Vec::new();

        if let Some(hits) = data["hits"]["hits"].as_array() {
            for hit in hits {
                if let Some(message) = hit["_source"]["message"].as_str() {
                    // println!("Event13 raw message: {}", message);
                    let mut entry = Event13 {
                        agent_name: None,
                        agent_id: None,
                        event_action: Some("Registry value set".to_string()),
                        event_type: None,
                        utc_time: None,
                        process_guid: None,
                        process_id: None,
                        image: None,
                        target_object: None,
                        details: None,
                        user: None,
                    };

                    if let Some(agent_name) = hit["_source"]["agent"]["name"].as_str() {
                        entry.agent_name = Some(agent_name.to_string());
                    }

                    if let Some(agent_id) = hit["_source"]["agent"]["id"].as_str() {
                        entry.agent_id = Some(agent_id.to_string());
                    }

                    for part in message.split('\n') {
                        let segments: Vec<_> = part.splitn(2, ':').collect();
                        // println!("{:?}", segments); // Debug prints
                        if segments.len() == 2 {
                            let key = segments[0].trim();
                            let value = segments[1].trim();
                            match key {
                                "EventType" => entry.event_type = Some(value.to_string()),
                                "UtcTime" => entry.utc_time = Some(value.to_string()),
                                "ProcessGuid" => entry.process_guid = Some(value.to_string()),
                                "ProcessId" => entry.process_id = Some(value.to_string()),
                                "Image" => entry.image = Some(value.to_string()),
                                "TargetObject" => entry.target_object = Some(value.to_string()),
                                "Details" => entry.details = Some(value.to_string()),
                                "User" => entry.user = Some(value.to_string()),
                                _ => {}
                            }
                        }
                    }

                    entries.push(entry);
                }
            }
        }

        entries
    }

    fn write_to_csv(entries: &Vec<Self>, filename: &str) -> io::Result<()> {
        // Check if entries is empty, and if so, return early.
        if entries.is_empty() {
            return Ok(());
        }

        let file_exists = fs::metadata(filename).is_ok();

        let mut wtr = if file_exists {
            // Open the file in append mode if it exists.
            csv::WriterBuilder::new()
                .delimiter(b'\t')
                .has_headers(false) // Don't write headers when appending.
                .from_writer(fs::OpenOptions::new().append(true).open(filename)?)
        } else {
            // Create a new file if it doesn't exist.
            csv::WriterBuilder::new()
                .delimiter(b'\t')
                .from_path(filename)?
        };

        for entry in entries {
            wtr.serialize(entry)?;
        }
        wtr.flush()?;
        Ok(())
    }
}

impl EventToCSV for Event14 {
    fn parse(data: &serde_json::Value) -> Vec<Self> {
        let mut entries = Vec::new();

        if let Some(hits) = data["hits"]["hits"].as_array() {
            for hit in hits {
                if let Some(message) = hit["_source"]["message"].as_str() {
                    // println!("Event14 raw message: {}", message);
                    let mut entry = Event14 {
                        agent_name: None,
                        agent_id: None,
                        event_action: Some("Registry value set".to_string()),
                        event_type: None,
                        utc_time: None,
                        process_guid: None,
                        process_id: None,
                        image: None,
                        target_object: None,
                        new_name: None,
                        user: None,
                    };

                    if let Some(agent_name) = hit["_source"]["agent"]["name"].as_str() {
                        entry.agent_name = Some(agent_name.to_string());
                    }

                    if let Some(agent_id) = hit["_source"]["agent"]["id"].as_str() {
                        entry.agent_id = Some(agent_id.to_string());
                    }

                    for part in message.split('\n') {
                        let segments: Vec<_> = part.splitn(2, ':').collect();
                        // println!("{:?}", segments); // Debug prints
                        if segments.len() == 2 {
                            let key = segments[0].trim();
                            let value = segments[1].trim();
                            match key {
                                "EventType" => entry.event_type = Some(value.to_string()),
                                "UtcTime" => entry.utc_time = Some(value.to_string()),
                                "ProcessGuid" => entry.process_guid = Some(value.to_string()),
                                "ProcessId" => entry.process_id = Some(value.to_string()),
                                "Image" => entry.image = Some(value.to_string()),
                                "TargetObject" => entry.target_object = Some(value.to_string()),
                                "NewName" => entry.new_name = Some(value.to_string()),
                                "User" => entry.user = Some(value.to_string()),
                                _ => {}
                            }
                        }
                    }

                    entries.push(entry);
                }
            }
        }

        entries
    }

    fn write_to_csv(entries: &Vec<Self>, filename: &str) -> io::Result<()> {
        // Check if entries is empty, and if so, return early.
        if entries.is_empty() {
            return Ok(());
        }

        let file_exists = fs::metadata(filename).is_ok();

        let mut wtr = if file_exists {
            // Open the file in append mode if it exists.
            csv::WriterBuilder::new()
                .delimiter(b'\t')
                .has_headers(false) // Don't write headers when appending.
                .from_writer(fs::OpenOptions::new().append(true).open(filename)?)
        } else {
            // Create a new file if it doesn't exist.
            csv::WriterBuilder::new()
                .delimiter(b'\t')
                .from_path(filename)?
        };

        for entry in entries {
            wtr.serialize(entry)?;
        }
        wtr.flush()?;
        Ok(())
    }
}

impl EventToCSV for Event15 {
    fn parse(data: &serde_json::Value) -> Vec<Self> {
        let mut entries = Vec::new();

        if let Some(hits) = data["hits"]["hits"].as_array() {
            for hit in hits {
                if let Some(message) = hit["_source"]["message"].as_str() {
                    // println!("Event15 raw message: {}", message);
                    let mut entry = Event15 {
                        agent_name: None,
                        agent_id: None,
                        event_action: Some("File stream created".to_string()),
                        utc_time: None,
                        process_guid: None,
                        process_id: None,
                        image: None,
                        target_filename: None,
                        creation_utc_time: None,
                        hash: None,
                        contents: None,
                        user: None,
                    };

                    if let Some(agent_name) = hit["_source"]["agent"]["name"].as_str() {
                        entry.agent_name = Some(agent_name.to_string());
                    }

                    if let Some(agent_id) = hit["_source"]["agent"]["id"].as_str() {
                        entry.agent_id = Some(agent_id.to_string());
                    }

                    for part in message.split('\n') {
                        let segments: Vec<_> = part.splitn(2, ':').collect();
                        // println!("{:?}", segments); // Debug prints
                        if segments.len() == 2 {
                            let key = segments[0].trim();
                            let value = segments[1].trim();
                            match key {
                                "UtcTime" => entry.utc_time = Some(value.to_string()),
                                "ProcessGuid" => entry.process_guid = Some(value.to_string()),
                                "ProcessId" => entry.process_id = Some(value.to_string()),
                                "Image" => entry.image = Some(value.to_string()),
                                "TargetFilename" => entry.target_filename = Some(value.to_string()),
                                "CreationUtcTime" => {
                                    entry.creation_utc_time = Some(value.to_string())
                                }
                                "Hash" => entry.hash = Some(value.to_string()),
                                "Contents" => entry.contents = Some(value.to_string()),
                                "User" => entry.user = Some(value.to_string()),
                                _ => {}
                            }
                        }
                    }

                    entries.push(entry);
                }
            }
        }

        entries
    }

    fn write_to_csv(entries: &Vec<Self>, filename: &str) -> io::Result<()> {
        // Check if entries is empty, and if so, return early.
        if entries.is_empty() {
            return Ok(());
        }

        let file_exists = fs::metadata(filename).is_ok();

        let mut wtr = if file_exists {
            // Open the file in append mode if it exists.
            csv::WriterBuilder::new()
                .delimiter(b'\t')
                .has_headers(false) // Don't write headers when appending.
                .from_writer(fs::OpenOptions::new().append(true).open(filename)?)
        } else {
            // Create a new file if it doesn't exist.
            csv::WriterBuilder::new()
                .delimiter(b'\t')
                .from_path(filename)?
        };

        for entry in entries {
            wtr.serialize(entry)?;
        }
        wtr.flush()?;
        Ok(())
    }
}

impl EventToCSV for Event17 {
    fn parse(data: &serde_json::Value) -> Vec<Self> {
        let mut entries = Vec::new();

        if let Some(hits) = data["hits"]["hits"].as_array() {
            for hit in hits {
                if let Some(message) = hit["_source"]["message"].as_str() {
                    // println!("Event17 raw message: {}", message);
                    let mut entry = Event17 {
                        agent_name: None,
                        agent_id: None,
                        event_action: Some("Pipe Created".to_string()),
                        utc_time: None,
                        event_type: None,
                        process_guid: None,
                        process_id: None,
                        pipe_name: None,
                        image: None,
                        user: None,
                    };

                    if let Some(agent_name) = hit["_source"]["agent"]["name"].as_str() {
                        entry.agent_name = Some(agent_name.to_string());
                    }

                    if let Some(agent_id) = hit["_source"]["agent"]["id"].as_str() {
                        entry.agent_id = Some(agent_id.to_string());
                    }

                    for part in message.split('\n') {
                        let segments: Vec<_> = part.splitn(2, ':').collect();
                        // println!("{:?}", segments); // Debug prints
                        if segments.len() == 2 {
                            let key = segments[0].trim();
                            let value = segments[1].trim();
                            match key {
                                "UtcTime" => entry.utc_time = Some(value.to_string()),
                                "EventType" => entry.event_type = Some(value.to_string()),
                                "ProcessGuid" => entry.process_guid = Some(value.to_string()),
                                "ProcessId" => entry.process_id = Some(value.to_string()),
                                "PipeName" => entry.pipe_name = Some(value.to_string()),
                                "Image" => entry.image = Some(value.to_string()),
                                "User" => entry.user = Some(value.to_string()),
                                _ => {}
                            }
                        }
                    }

                    entries.push(entry);
                }
            }
        }

        entries
    }

    fn write_to_csv(entries: &Vec<Self>, filename: &str) -> io::Result<()> {
        // Check if entries is empty, and if so, return early.
        if entries.is_empty() {
            return Ok(());
        }

        let file_exists = fs::metadata(filename).is_ok();

        let mut wtr = if file_exists {
            // Open the file in append mode if it exists.
            csv::WriterBuilder::new()
                .delimiter(b'\t')
                .has_headers(false) // Don't write headers when appending.
                .from_writer(fs::OpenOptions::new().append(true).open(filename)?)
        } else {
            // Create a new file if it doesn't exist.
            csv::WriterBuilder::new()
                .delimiter(b'\t')
                .from_path(filename)?
        };

        for entry in entries {
            wtr.serialize(entry)?;
        }
        wtr.flush()?;
        Ok(())
    }
}

impl EventToCSV for Event18 {
    fn parse(data: &serde_json::Value) -> Vec<Self> {
        let mut entries = Vec::new();

        if let Some(hits) = data["hits"]["hits"].as_array() {
            for hit in hits {
                if let Some(message) = hit["_source"]["message"].as_str() {
                    // println!("Event18 raw message: {}", message);
                    let mut entry = Event18 {
                        agent_name: None,
                        agent_id: None,
                        event_action: Some("Pipe Connected".to_string()),
                        utc_time: None,
                        event_type: None,
                        process_guid: None,
                        process_id: None,
                        pipe_name: None,
                        image: None,
                        user: None,
                    };

                    if let Some(agent_name) = hit["_source"]["agent"]["name"].as_str() {
                        entry.agent_name = Some(agent_name.to_string());
                    }

                    if let Some(agent_id) = hit["_source"]["agent"]["id"].as_str() {
                        entry.agent_id = Some(agent_id.to_string());
                    }

                    for part in message.split('\n') {
                        let segments: Vec<_> = part.splitn(2, ':').collect();
                        // println!("{:?}", segments); // Debug prints
                        if segments.len() == 2 {
                            let key = segments[0].trim();
                            let value = segments[1].trim();
                            match key {
                                "UtcTime" => entry.utc_time = Some(value.to_string()),
                                "EventType" => entry.event_type = Some(value.to_string()),
                                "ProcessGuid" => entry.process_guid = Some(value.to_string()),
                                "ProcessId" => entry.process_id = Some(value.to_string()),
                                "PipeName" => entry.pipe_name = Some(value.to_string()),
                                "Image" => entry.image = Some(value.to_string()),
                                "User" => entry.user = Some(value.to_string()),
                                _ => {}
                            }
                        }
                    }

                    entries.push(entry);
                }
            }
        }

        entries
    }

    fn write_to_csv(entries: &Vec<Self>, filename: &str) -> io::Result<()> {
        // Check if entries is empty, and if so, return early.
        if entries.is_empty() {
            return Ok(());
        }

        let file_exists = fs::metadata(filename).is_ok();

        let mut wtr = if file_exists {
            // Open the file in append mode if it exists.
            csv::WriterBuilder::new()
                .delimiter(b'\t')
                .has_headers(false) // Don't write headers when appending.
                .from_writer(fs::OpenOptions::new().append(true).open(filename)?)
        } else {
            // Create a new file if it doesn't exist.
            csv::WriterBuilder::new()
                .delimiter(b'\t')
                .from_path(filename)?
        };

        for entry in entries {
            wtr.serialize(entry)?;
        }
        wtr.flush()?;
        Ok(())
    }
}

impl EventToCSV for Event22 {
    fn parse(data: &serde_json::Value) -> Vec<Self> {
        let mut entries = Vec::new();

        if let Some(hits) = data["hits"]["hits"].as_array() {
            for hit in hits {
                if let Some(message) = hit["_source"]["message"].as_str() {
                    // println!("Event22 raw message: {}", message);
                    let mut entry = Event22 {
                        agent_name: None,
                        agent_id: None,
                        event_action: Some("Dns query".to_string()),
                        utc_time: None,
                        process_guid: None,
                        process_id: None,
                        query_name: None,
                        query_status: None,
                        query_results: None,
                        image: None,
                        user: None,
                    };

                    if let Some(agent_name) = hit["_source"]["agent"]["name"].as_str() {
                        entry.agent_name = Some(agent_name.to_string());
                    }

                    if let Some(agent_id) = hit["_source"]["agent"]["id"].as_str() {
                        entry.agent_id = Some(agent_id.to_string());
                    }

                    for part in message.split('\n') {
                        let segments: Vec<_> = part.splitn(2, ':').collect();
                        // println!("{:?}", segments); // Debug prints
                        if segments.len() == 2 {
                            let key = segments[0].trim();
                            let value = segments[1].trim();
                            match key {
                                "UtcTime" => entry.utc_time = Some(value.to_string()),
                                "ProcessGuid" => entry.process_guid = Some(value.to_string()),
                                "ProcessId" => entry.process_id = Some(value.to_string()),
                                "QueryName" => entry.query_name = Some(value.to_string()),
                                "QueryStatus" => entry.query_status = Some(value.to_string()),
                                "QueryResults" => entry.query_results = Some(value.to_string()),
                                "Image" => entry.image = Some(value.to_string()),
                                "User" => entry.user = Some(value.to_string()),
                                _ => {}
                            }
                        }
                    }

                    entries.push(entry);
                }
            }
        }

        entries
    }

    fn write_to_csv(entries: &Vec<Self>, filename: &str) -> io::Result<()> {
        // Check if entries is empty, and if so, return early.
        if entries.is_empty() {
            return Ok(());
        }

        let file_exists = fs::metadata(filename).is_ok();

        let mut wtr = if file_exists {
            // Open the file in append mode if it exists.
            csv::WriterBuilder::new()
                .delimiter(b'\t')
                .has_headers(false) // Don't write headers when appending.
                .from_writer(fs::OpenOptions::new().append(true).open(filename)?)
        } else {
            // Create a new file if it doesn't exist.
            csv::WriterBuilder::new()
                .delimiter(b'\t')
                .from_path(filename)?
        };

        for entry in entries {
            wtr.serialize(entry)?;
        }
        wtr.flush()?;
        Ok(())
    }
}

impl EventToCSV for Event23 {
    fn parse(data: &serde_json::Value) -> Vec<Self> {
        let mut entries = Vec::new();

        if let Some(hits) = data["hits"]["hits"].as_array() {
            for hit in hits {
                if let Some(message) = hit["_source"]["message"].as_str() {
                    // println!("Event23 raw message: {}", message);
                    let mut entry = Event23 {
                        agent_name: None,
                        agent_id: None,
                        event_action: Some("File Delete archived,".to_string()),
                        utc_time: None,
                        process_guid: None,
                        process_id: None,
                        user: None,
                        image: None,
                        target_filename: None,
                        hashes: None,
                        is_executable: None,
                        archived: None,
                    };

                    if let Some(agent_name) = hit["_source"]["agent"]["name"].as_str() {
                        entry.agent_name = Some(agent_name.to_string());
                    }

                    if let Some(agent_id) = hit["_source"]["agent"]["id"].as_str() {
                        entry.agent_id = Some(agent_id.to_string());
                    }

                    for part in message.split('\n') {
                        let segments: Vec<_> = part.splitn(2, ':').collect();
                        // println!("{:?}", segments); // Debug prints
                        if segments.len() == 2 {
                            let key = segments[0].trim();
                            let value = segments[1].trim();
                            match key {
                                "UtcTime" => entry.utc_time = Some(value.to_string()),
                                "ProcessGuid" => entry.process_guid = Some(value.to_string()),
                                "ProcessId" => entry.process_id = Some(value.to_string()),
                                "User" => entry.user = Some(value.to_string()),
                                "Image" => entry.image = Some(value.to_string()),
                                "TargetFilename" => entry.target_filename = Some(value.to_string()),
                                "Hashes" => entry.hashes = Some(value.to_string()),
                                "IsExecutable" => entry.is_executable = Some(value.to_string()),
                                "Archived" => entry.archived = Some(value.to_string()),
                                _ => {}
                            }
                        }
                    }

                    entries.push(entry);
                }
            }
        }

        entries
    }

    fn write_to_csv(entries: &Vec<Self>, filename: &str) -> io::Result<()> {
        // Check if entries is empty, and if so, return early.
        if entries.is_empty() {
            return Ok(());
        }

        let file_exists = fs::metadata(filename).is_ok();

        let mut wtr = if file_exists {
            // Open the file in append mode if it exists.
            csv::WriterBuilder::new()
                .delimiter(b'\t')
                .has_headers(false) // Don't write headers when appending.
                .from_writer(fs::OpenOptions::new().append(true).open(filename)?)
        } else {
            // Create a new file if it doesn't exist.
            csv::WriterBuilder::new()
                .delimiter(b'\t')
                .from_path(filename)?
        };

        for entry in entries {
            wtr.serialize(entry)?;
        }
        wtr.flush()?;
        Ok(())
    }
}

impl EventToCSV for Event25 {
    fn parse(data: &serde_json::Value) -> Vec<Self> {
        let mut entries = Vec::new();

        if let Some(hits) = data["hits"]["hits"].as_array() {
            for hit in hits {
                if let Some(message) = hit["_source"]["message"].as_str() {
                    // println!("Event25 raw message: {}", message);
                    let mut entry = Event25 {
                        agent_name: None,
                        agent_id: None,
                        event_action: Some("Process Tampering,".to_string()),
                        utc_time: None,
                        process_guid: None,
                        process_id: None,
                        image: None,
                        types: None,
                        user: None,
                    };

                    if let Some(agent_name) = hit["_source"]["agent"]["name"].as_str() {
                        entry.agent_name = Some(agent_name.to_string());
                    }

                    if let Some(agent_id) = hit["_source"]["agent"]["id"].as_str() {
                        entry.agent_id = Some(agent_id.to_string());
                    }

                    for part in message.split('\n') {
                        let segments: Vec<_> = part.splitn(2, ':').collect();
                        // println!("{:?}", segments); // Debug prints
                        if segments.len() == 2 {
                            let key = segments[0].trim();
                            let value = segments[1].trim();
                            match key {
                                "UtcTime" => entry.utc_time = Some(value.to_string()),
                                "ProcessGuid" => entry.process_guid = Some(value.to_string()),
                                "ProcessId" => entry.process_id = Some(value.to_string()),
                                "Image" => entry.image = Some(value.to_string()),
                                "Type" => entry.types = Some(value.to_string()),
                                "User" => entry.user = Some(value.to_string()),
                                _ => {}
                            }
                        }
                    }

                    entries.push(entry);
                }
            }
        }

        entries
    }

    fn write_to_csv(entries: &Vec<Self>, filename: &str) -> io::Result<()> {
        // Check if entries is empty, and if so, return early.
        if entries.is_empty() {
            return Ok(());
        }

        let file_exists = fs::metadata(filename).is_ok();

        let mut wtr = if file_exists {
            // Open the file in append mode if it exists.
            csv::WriterBuilder::new()
                .delimiter(b'\t')
                .has_headers(false) // Don't write headers when appending.
                .from_writer(fs::OpenOptions::new().append(true).open(filename)?)
        } else {
            // Create a new file if it doesn't exist.
            csv::WriterBuilder::new()
                .delimiter(b'\t')
                .from_path(filename)?
        };

        for entry in entries {
            wtr.serialize(entry)?;
        }
        wtr.flush()?;
        Ok(())
    }
}

impl EventToCSV for Event26 {
    fn parse(data: &serde_json::Value) -> Vec<Self> {
        let mut entries = Vec::new();

        if let Some(hits) = data["hits"]["hits"].as_array() {
            for hit in hits {
                if let Some(message) = hit["_source"]["message"].as_str() {
                    // println!("Event26 raw message: {}", message);
                    let mut entry = Event26 {
                        agent_name: None,
                        agent_id: None,
                        event_action: Some("File Delete logged,".to_string()),
                        utc_time: None,
                        process_guid: None,
                        process_id: None,
                        user: None,
                        image: None,
                        target_filename: None,
                        hashes: None,
                        is_executable: None,
                    };

                    if let Some(agent_name) = hit["_source"]["agent"]["name"].as_str() {
                        entry.agent_name = Some(agent_name.to_string());
                    }

                    if let Some(agent_id) = hit["_source"]["agent"]["id"].as_str() {
                        entry.agent_id = Some(agent_id.to_string());
                    }

                    for part in message.split('\n') {
                        let segments: Vec<_> = part.splitn(2, ':').collect();
                        // println!("{:?}", segments); // Debug prints
                        if segments.len() == 2 {
                            let key = segments[0].trim();
                            let value = segments[1].trim();
                            match key {
                                "UtcTime" => entry.utc_time = Some(value.to_string()),
                                "ProcessGuid" => entry.process_guid = Some(value.to_string()),
                                "ProcessId" => entry.process_id = Some(value.to_string()),
                                "User" => entry.user = Some(value.to_string()),
                                "Image" => entry.image = Some(value.to_string()),
                                "TargetFilename" => entry.target_filename = Some(value.to_string()),
                                "Hashes" => entry.hashes = Some(value.to_string()),
                                "IsExecutable" => entry.is_executable = Some(value.to_string()),
                                _ => {}
                            }
                        }
                    }

                    entries.push(entry);
                }
            }
        }

        entries
    }

    fn write_to_csv(entries: &Vec<Self>, filename: &str) -> io::Result<()> {
        // Check if entries is empty, and if so, return early.
        if entries.is_empty() {
            return Ok(());
        }

        let file_exists = fs::metadata(filename).is_ok();

        let mut wtr = if file_exists {
            // Open the file in append mode if it exists.
            csv::WriterBuilder::new()
                .delimiter(b'\t')
                .has_headers(false) // Don't write headers when appending.
                .from_writer(fs::OpenOptions::new().append(true).open(filename)?)
        } else {
            // Create a new file if it doesn't exist.
            csv::WriterBuilder::new()
                .delimiter(b'\t')
                .from_path(filename)?
        };

        for entry in entries {
            wtr.serialize(entry)?;
        }
        wtr.flush()?;
        Ok(())
    }
}
