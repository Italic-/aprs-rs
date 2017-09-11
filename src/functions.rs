//! Frame parsers and data handlers
//!
//! Parse various frame types and return a fully-constructed object. See individual functions for
//! their descriptions.

use structs::{Frame, Callsign, InformationField};
use constants;


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
    let mut _frame: &[u8] = raw_frame;
    let mut kiss_call: bool = false;

    if _frame.starts_with(&[constants::KISS_DATA_FRAME]) {
        let mut ind: usize = 0;
        for (id, byt) in _frame.iter().enumerate() {
            if byt == &constants::KISS_DATA_FRAME {
                continue;
            } else {
                ind = id;
                break;
            }
        }
        _frame = &_frame[ind..];
        kiss_call = true;
    }
    if _frame.ends_with(&[constants::KISS_DATA_FRAME]) {
        let mut ind: usize = 0;
        for (id, byt) in _frame.iter().rev().enumerate() {
            if byt == &constants::KISS_DATA_FRAME {
                continue;
            } else {
                ind = id;
                break;
            }
        }
        _frame = &_frame[..(_frame.len() - ind)];
        kiss_call = true;
    }

    let mut info_pos: usize = 0;
    for win in _frame.windows(2) {
        if win == constants::ADDR_INFO_DELIM {
            break;
        }
        info_pos += 1;
    }
    let (frame_addressing, _) = _frame.split_at(info_pos);
    let (_, mut info_field)   = _frame.split_at(info_pos + 2);
    if info_field.ends_with(&[0xff, 0x07]) {
        info_field = &info_field[..(info_field.len() - 2)];
    }
    let dest: Callsign = parse_callsign_ax25(&frame_addressing, kiss_call);
    let src: Callsign = parse_callsign_ax25(&frame_addressing[7..], kiss_call);

    let mut path: Vec<Callsign> = Vec::new();
    for chunk in frame_addressing[14..].chunks(7) {
        path.push(parse_callsign_ax25(chunk, false));
    }

    Frame {
        source: src,
        destination: dest,
        path: path,
        info: parse_info_field(info_field),
    }
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
    // based on ax.25 v2.0 protocol, NOT v2.2
    let mut _callsign: Vec<u8> = Vec::new();
    let mut digi: bool = false;

    for chunk in raw_callsign[..6].iter() {
        let mut _chunk = chunk & 0xFF;
        // shift 1 bit
        _chunk = _chunk >> 1;
        if (_chunk as char).is_alphanumeric() {
            _callsign.push(_chunk);
        }
    }

    // seventh byte is ssid or digi
    let seven_chunk: u8 = raw_callsign[6] & 0xFF;
    let ssid = (seven_chunk >> 1) & 0x0F;

    // fixme - see python module
    if kiss_call {
        if (seven_chunk >> 1 & 0x80) == 1 {
            digi = true;
        }
    } else {
        if (seven_chunk & 0x80) == 1 {
            digi = true;
        }
    }
    Callsign {
        callsign: String::from_utf8(_callsign).unwrap(),
        ssid: ssid,
        digi: digi
    }
}

pub fn parse_info_field(raw_data: &[u8]) -> InformationField {
    let first_byte: &u8 = raw_data.first().unwrap();
    let data_type: String = constants::DATA_TYPE_MAP.get(first_byte).unwrap().to_string();
    InformationField {
        data: raw_data.to_vec(),
        data_type: data_type,
        safe: true
    }
}

pub fn default_data_handler(data: &[u8], data_type: u8) -> InformationField {
    let _data_type: String = constants::DATA_TYPE_MAP.get(&data_type).unwrap().to_string();
    InformationField {
        data: data.to_vec(),
        data_type: _data_type,
        safe: false
    }
}
