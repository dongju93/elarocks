#![allow(dead_code)]

use super::elastic::*;

// SECRET information
pub const ES_URL: &str = ES_URL_SECRET;
pub const ID: &str = ID_SECRET;
pub const PW: &str = PW_SECRET;

// INDICES for src/main.rs
pub const INDICES: [&str; 1] = [".ds-winlogbeat-8.8.2-2023.08.06-000001"];

// Search start and end timestamp
pub const TIMESTAMP_STA: &str = "2023-08-10T15:00:00.000Z";
pub const TIMESTAMP: &str = "2023-08-11T15:00:00.000Z";

// Query size
pub const SIZE: usize = 10000;

// File save location
pub const SAVELOCATION: &str = "/Users/dong-ju/Documents/My_code/elarocks/file/temp/event";
pub const CSVNAME: &str = "_logs.csv";
