//! Constants for use throughout the library.
//!
//! Constants include APRS-IS servers, HTTP headers, AX.25 control bytes, symbol mapping, and KISS
//! data.

use std::collections::HashMap;

/// Array of main APRS-IS servers
pub static APRSIS_SERVERS: &'static [&'static str; 7] = &[
    "rotate.aprs.net",
    "rotate.aprs2.net",
    "noam.aprs2.net",
    "euro.aprs2.net",
    "aunz.aprs2.net",
    "soam.aprs2.net",
    "asia.aprs2.net"
];
/// Name of libary to present
pub static APRSIS_SW_VERSION: &'static str = "APRSrust";

/// HTTP headers
lazy_static! {
    pub static ref APRSIS_HTTP_HEADERS: HashMap<&'static str, &'static str> = {
        let mut h: HashMap<&'static str, &'static str> = HashMap::new();
        h.insert("content-type", "application/octet-stream");
        h.insert("accept", "text/plain");
        h
    };
}

pub const APRSIS_FILTER_PORT: usize = 14580;
pub const APRSIS_RX_PORT: usize = 8080;
pub static APRSIS_URL: &'static str = "http://srvr.aprs-is.net:8080";

pub const RECV_BUFFER: usize = 1024;

pub static DEFAULT_TOCALL: &'static str = "APYT70";

/// AX.25 Flag - The flag field at each end of the frame is the bit sequence 0x7E
/// that separates each frame.
pub const AX25_FLAG: u8 = 0x7E;
/// AX.25 Control Field - This field is set to 0x03 (UI-frame).
pub const AX25_CONTROL_FIELD: u8 = 0x03;
/// AX.25 Protocol ID - This field is set to 0xF0 (no layer 3 protocol).
pub const AX25_PROTOCOL_ID: u8 = 0xF0;
/// A good place to split AX.25 address from information fields.
pub const ADDR_INFO_DELIM: &'static [u8] = &[0x03, 0xF0];

/// Data symbol map
lazy_static! {
    pub static ref DATA_TYPE_MAP: HashMap<u8, &'static str> = {
        let mut d: HashMap<u8, &'static str> = HashMap::new();
        d.insert(b'>', "status");
        d.insert(b'!', "position_nots_nomsg");
        d.insert(b'=', "position_nots_msg");
        d.insert(b'T', "telemetry");
        d.insert(b';', "object");
        d.insert(b'`', "old_mice");
        d
    };
}

/// KISS Command Codes
/// https://en.wikipedia.org/wiki/KISS_(TNC)#Command_Codes
pub const KISS_DATA_FRAME: u8 = 0x00;
