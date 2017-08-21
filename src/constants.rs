use std::collections::HashMap;

let APRSIS_SERVERS: Vec<&str> = vec!["rotate.aprs.net", "noam.aprs2.net"];
let APRSIS_SW_VERSION: String = "APRS Rust Module".to_string();
let APRSIS_HTTP_HEADERS: HashMap<&str, &str> = HashMap::new()
    .insert("content-type", "application/octet-stream")
    .insert("accept", "text/plain");
let APRISIS_FILTER_PORT: usize = 14580;
let APRSIS_RX_PORT: usize = 8080;
let APRSIS_URL: String = "http://srvr.aprs-is.net:8080".to_string();

let RECV_BUFFER: usize = 1024;

let DEFAULT_TOCALL: String = "APYT70".to_string();

// AX.25 Flag - The flag field at each end of the frame is the bit sequence 0x7E that separates
// each frame.
let AX25_FLAG: String = "\x7E".to_string();
// AX.25 Control Field - This field is set to 0x03 (UI-frame).
let AX25_CONTROL_FIELD: String = "\x03".to_string();
// AX.25 Protocol ID - This field is set to 0xF0 (no layer 3 protocol).
// let AX25_PROTOCOL_ID: String = "\xF0".to_string();
// A good place to split AX.25 address from information fields.
let ADDR_INFO_DELIM: String = &AX25_CONTROL_FIELD + &AX25_PROTOCOL_ID;

let DATA_TYPE_MAP: HashMap<&str, &str> = HashMap::new()
    .insert(">", "status")
    .insert("!", "position_nots_nomsg")
    .insert("=", "position_nots_msg")
    .insert("T", "telemetry")
    .insert(";", "object")
    .insert("`", "old_mice");

// KISS Command Codes
// https://en.wikipedia.org/wiki/KISS_(TNC)#Command_Codes
let KISS_DATA_FRAME: String = "\x00".to_string();
