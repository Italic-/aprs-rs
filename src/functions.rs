//! Frame parsers and data handlers
//!
//! Parse various frame types and return a fully-constructed object. See individual functions for
//! their descriptions.

use structs::{Frame, Callsign, InformationField};
use constants;
use util;


pub fn parse_frame(raw_frame: &[u8]) -> Frame {
    for win in raw_frame.windows(2) {
        if win == constants::ADDR_INFO_DELIM {
            return parse_frame_ax25(raw_frame);
        }
    }
    parse_frame_text(raw_frame)
}

pub fn parse_frame_text(raw_frame: &[u8]) -> Frame {
    // src>dest[,path]:info
    let src_dest:  usize = raw_frame.iter().position(|x| *x == b'>').unwrap();
    let path_info: usize = raw_frame.iter().position(|x| *x == b':').unwrap();

    let source: Callsign = parse_callsign_text(&raw_frame[..src_dest]);
    let mut dest: Callsign = Callsign::new();
    let info: InformationField = parse_info_field(&raw_frame[(path_info + 1)..]);

    // Full path slice
    let mut _path: Vec<Callsign> = Vec::new();
    let _paths: &[u8] = &raw_frame[src_dest + 1..path_info];
    for (ind, path) in _paths.split(|x| *x == b',').enumerate() {
        if ind == 0 {
            dest = parse_callsign_text(path);
        } else {
            _path.push(parse_callsign_text(path));
        }
    }
    Frame {
        source: source,
        destination: dest,
        path: _path,
        info: info
    }
}

pub fn parse_frame_ax25(raw_frame: &[u8]) -> Frame {
    let parsed_frame: Frame = Frame::new();
    parsed_frame
}

pub fn parse_callsign(raw_callsign: &[u8]) -> Callsign {
    parse_callsign_text(raw_callsign)
}

pub fn parse_callsign_text(raw_callsign: &[u8]) -> Callsign {
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
    Callsign {
        callsign: _callsign,
        ssid: ssid,
        digi: digi
    }
}

pub fn parse_callsign_ax25(raw_callsign: &[u8], kiss_call: bool) -> Callsign {
    Callsign::new()
}

pub fn parse_info_field(raw_data: &[u8]) -> InformationField {
    // TODO: use char to get map entry
    let first_byte: &u8 = raw_data.first().unwrap();
    let data_type: String = constants::DATA_TYPE_MAP.get(first_byte).unwrap().to_string();
    InformationField {
        data: raw_data.to_vec(),
        data_type,
        safe: true
    }
}

pub fn default_data_handler(data: &[u8], data_type: &str) -> String {
    String::new()
}
