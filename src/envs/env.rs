#![allow(dead_code)]

use super::elastic::*;

// SECRET information
pub const ES_URL: &str = ES_URL_SECRET;
pub const ID: &str = ID_SECRET;
pub const PW: &str = PW_SECRET;

// INDICES for src/main.rs
pub const INDICES: [&str; 2] = [".ds-winlogbeat-8.8.1-2023.08.16-000001", ".ds-winlogbeat-8.8.2-2023.08.06-000001"];

// Search start and end timestamp
pub const TIMESTAMP_STA: &str = "2023-08-06T15:00:00.000Z";
pub const TIMESTAMP: &str = "2023-09-07T02:00:00.000Z";

// Query size
pub const SIZE: usize = 1000000;

// File save location
pub const SAVELOCATION: &str = "/Users/dong-ju/Documents/My_code/elarocks/file/temp/event";
pub const CSVNAME: &str = "_logs.csv";

pub const DBCONN: &str = "host=localhost user=dong-ju dbname=postgres";
pub const DBINSE_REG: &str = "INSERT INTO sysmon.reg_eve (savedtime) VALUES ($1)";
pub const DBINSE_PRO: &str = "INSERT INTO sysmon.pro_eve (savedtime) VALUES ($1)";