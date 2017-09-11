//! Various APRS data types
//!
//! Data is broken down into objects depending on its purpose. See each struct definition for its
//! description.

use constants;
use functions;
use geo_util;


pub struct Frame {
    pub source: Callsign,
    pub destination: Callsign,
    pub path: Vec<Callsign>,
    pub info: InformationField,
}

impl Frame {
    pub fn new() -> Frame {
        Frame {
            source: Callsign::new(),
            destination: Callsign::new(),
            path: Vec::new(),
            info: InformationField::new()
        }
    }
}

impl Frame {
    pub fn set_source(&mut self, source: &[u8]) {
        self.source = functions::parse_callsign(source);
    }
    pub fn set_destination(&mut self, dest: &[u8]) {
        self.destination = functions::parse_callsign(dest);
    }
    pub fn set_path(&mut self, path: Vec<Vec<u8>>) {
        for pth in path {
            let byts: &[u8] = pth.as_slice();
            self.path.push(functions::parse_callsign(byts));
        }
    }
    pub fn set_info(&mut self, info: &[u8]) {
        self.info = functions::parse_info_field(info);
    }
    pub fn encode_ax25(&self) -> Vec<u8> {
        // TODO: find a way to return raw bytes
        let mut encoded_frame: Vec<u8> = Vec::new();
        encoded_frame.push(constants::AX25_FLAG);
        encoded_frame.extend(self.destination.encode_ax25());
        encoded_frame.extend(self.source.encode_ax25());
        for path_call in self.path.iter() {
            encoded_frame.extend(path_call.encode_ax25());
        }
        encoded_frame.extend(self.info.data.as_slice());
        encoded_frame
    }
}

pub struct PositionFrame {
    pub source: Callsign,
    pub destination: Callsign,
    pub path: Vec<Callsign>,
    pub info: InformationField,
    pub comment: Vec<u8>,
    pub table: Vec<u8>,
    pub symbol: u8,
    pub lat: f32,
    pub lng: f32,
    pub ambiguity: usize
}

impl PositionFrame {
    pub fn new() -> PositionFrame {
        PositionFrame {
            source: Callsign::new(),
            destination: Callsign::new(),
            path: Vec::new(),
            info: InformationField::new(),
            comment: Vec::new(),
            table: Vec::new(),
            symbol: 0,
            lat: 0.0,
            lng: 0.0,
            ambiguity: 0
        }
    }
}

impl PositionFrame {
    pub fn set_source(&mut self, source: &[u8]) {
        self.source = functions::parse_callsign(source);
    }
    pub fn set_destination(&mut self, dest: &[u8]) {
        self.destination = functions::parse_callsign(dest);
    }
    pub fn set_path(&mut self, path: Vec<Vec<u8>>) {
        for pth in path {
            let byts: &[u8] = pth.as_slice();
            self.path.push(functions::parse_callsign(byts));
        }
    }
    pub fn create_info_field(&self) -> Vec<u8> {
        let lat = geo_util::dec2dm_lat(self.lat);
        let lat_enc = geo_util::ambiguate(&lat, self.ambiguity);
        let lng = geo_util::dec2dm_lng(self.lng);
        let lng_enc = geo_util::ambiguate(&lng, self.ambiguity);

        let mut frame: Vec<u8> = Vec::new();
        frame.push(b'=');
        frame.extend(lat_enc.as_bytes());
        frame.extend(&self.table);
        frame.extend(lng_enc.as_bytes());
        frame.push(self.symbol);
        frame.extend(&self.comment);

        frame
    }
}

pub struct Callsign {
    pub callsign: String,
    pub ssid: u8,
    pub digi: bool,
}

impl Callsign {
    pub fn new() -> Callsign {
        Callsign {
            callsign: String::new(),
            ssid: 0,
            digi: false,
        }
    }
}

impl Callsign {
    pub fn set_callsign(&mut self, callsign: String) {
        self.callsign = callsign;
        // self.callsign = String::from_utf8(callsign.to_vec()).unwrap();
    }
    pub fn set_ssid(&mut self, ssid: u8) {
        self.ssid = ssid;
    }
    pub fn set_digi(&mut self, digi: bool) {
        self.digi = digi;
    }
    pub fn encode_ax25(&self) -> Vec<u8> {
        // TODO: find a way to return raw bytes
        let mut encoded_callsign: Vec<u8> = Vec::new();
        let mut encoded_ssid = (self.ssid << 1) | 0x60;
        let mut _callsign: String = String::new();

        if self.digi {
            encoded_ssid = encoded_ssid | 0x80;
        }
        for chr in self.callsign.chars() {
            _callsign.push(chr);
        }
        while _callsign.len() < 6 {
            _callsign.push(' ');
        }
        for byt in _callsign.bytes() {
            encoded_callsign.push(byt << 1);
        }
        encoded_callsign.push(encoded_ssid);

        encoded_callsign
    }
}

pub struct InformationField {
    pub data: Vec<u8>,
    pub data_type: String,
    pub safe: bool,
}

impl InformationField {
    pub fn new() -> InformationField {
        InformationField {
            data: Vec::new(),
            data_type: String::new(),
            safe: false
        }
    }
}

impl InformationField {
    pub fn set_data(&mut self, raw_data: &[u8]) {
        self.data = raw_data.to_vec();
    }
    pub fn set_data_type(&mut self, data_type: &str) {
        self.data_type = data_type.to_string();
    }
    pub fn set_safe(&mut self, safe: bool) {
        self.safe = safe;
    }
}

pub struct APRS;

pub struct TCP;

pub struct UDP;

pub struct HTTP;
