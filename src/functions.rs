//! Frame parsers and data handlers
//!
//! Parse various frame types and return a fully-constructed object. See individual functions for
//! their descriptions.

use structs;
use constants;
use util;


pub fn parse_frame(raw_frame: &[u8]) -> structs::Frame {
    for win in raw_frame.windows(2) {
        if win == constants::ADDR_INFO_DELIM {
            return parse_frame_ax25(raw_frame);
        }
    }
    parse_frame_text(raw_frame)
}

pub fn parse_frame_text(raw_frame: &[u8]) -> structs::Frame {
    let parsed_frame: structs::Frame = structs::Frame::new();
    parsed_frame
}

pub fn parse_frame_ax25(raw_frame: &[u8]) -> structs::Frame {
    let parsed_frame: structs::Frame = structs::Frame::new();
    parsed_frame
}

pub fn parse_callsign(raw_callsign: &[u8]) -> structs::Callsign {
    parse_callsign_text(raw_callsign)
}

pub fn parse_callsign_text(raw_callsign: &[u8]) -> structs::Callsign {
    let mut _callsign: String = String::from_utf8(raw_callsign.to_vec()).unwrap();
    let mut ssid: u8 = 0;
    let mut digi: bool = false;

    if _callsign.contains('*') {
        _callsign.trim_matches('*');
        digi = true;
    }
    let pos: Option<usize> = _callsign.find('-');
    match pos {
        Some(x) => {
            _callsign = {
                let (call, id): (&str, &str) = _callsign.split_at(x);
                ssid = id[1..].parse::<u8>().unwrap();
                call.to_string()
            };
        },
        None => {}
    }
    structs::Callsign {
        callsign: _callsign,
        ssid: ssid,
        digi: digi
    }
}

pub fn parse_callsign_ax25(raw_callsign: &[u8], kiss_call: bool) -> structs::Callsign {
    structs::Callsign::new()
}

pub fn parse_info_field(raw_data: &[u8]) -> structs::InformationField {
    // TODO: use char to get map entry
    let first_byte: &u8 = raw_data.first().unwrap();
    let data_type: String = constants::DATA_TYPE_MAP.get(first_byte).unwrap().to_string();
    structs::InformationField{data: raw_data.to_vec(), data_type, safe: true}
}

pub fn default_data_handler(data: &[u8], data_type: &str) -> String {
    String::new()
}
