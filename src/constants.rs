use std::collections::HashMap;

pub static APRSIS_SERVERS: Vec<&'static str> = ["rotate.aprs.net", "noam.aprs2.net"].to_vec();
pub static APRSIS_SW_VERSION: &'static str = "APRS Rust Module";

lazy_static! {
    pub static ref APRSIS_HTTP_HEADERS: HashMap<&'static str, &'static str> = {
        let mut h = HashMap::new();
        h.insert("content-type", "application/octet-stream");
        h.insert("accept", "text/plain");
        h
    };
}

pub static APRSIS_FILTER_PORT: usize = 14580;
pub static APRSIS_RX_PORT: usize = 8080;
pub static APRSIS_URL: &'static str = "http://srvr.aprs-is.net:8080";

pub static RECV_BUFFER: usize = 1024;

pub static DEFAULT_TOCALL: &'static str = "APYT70";

/// AX.25 Flag - The flag field at each end of the frame is the
///              bit sequence 0x7E that separates each frame.
pub static AX25_FLAG: u8 = 0x7E;
/// AX.25 Control Field - This field is set to 0x03 (UI-frame).
pub static AX25_CONTROL_FIELD: u8 = 0x03;
/// AX.25 Protocol ID - This field is set to 0xF0 (no layer 3 protocol).
pub static AX25_PROTOCOL_ID: u8 = 0xF0;
/// A good place to split AX.25 address from information fields.
pub static ADDR_INFO_DELIM: String = format!("{}{}", &AX25_CONTROL_FIELD, &AX25_PROTOCOL_ID);

lazy_static! {
    pub static ref DATA_TYPE_MAP: HashMap<&'static str, &'static str> = {
        let mut d = HashMap::new();
        d.insert(">", "status");
        d.insert("!", "position_nots_nomsg");
        d.insert("=", "position_nots_msg");
        d.insert("T", "telemetry");
        d.insert(";", "object");
        d.insert("`", "old_mice");
        d
    };
}

/// KISS Command Codes
/// https://en.wikipedia.org/wiki/KISS_(TNC)#Command_Codes
pub static KISS_DATA_FRAME: u8 = 0x00;
