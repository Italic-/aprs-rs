#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

pub mod constants;
pub use constants::{
    // LOG_FORMAT, LOG_LEVEL,
    APRSIS_SW_VERSION, APRSIS_HTTP_HEADERS, APRSIS_SERVERS,
    APRSIS_FILTER_PORT, APRSIS_RX_PORT, APRSIS_URL,
    AX25_FLAG, AX25_CONTROL_FIELD, AX25_PROTOCOL_ID,
    RECV_BUFFER, DEFAULT_TOCALL,
    ADDR_INFO_DELIM, DATA_TYPE_MAP, KISS_DATA_FRAME
};

pub mod util;
pub use util::{valid_callsign};

pub mod geo_util;
pub use geo_util::{dec2dm_lat, dec2dm_lng, ambiguate};

pub mod fcs;
pub use fcs::{FCS};

pub mod functions;
pub use functions::{
    parse_frame, parse_callsign,
    parse_callsign_ax25, parse_info_field
};

pub mod structs;
pub use structs::{
    Frame, Callsign, APRS, TCP, UDP,
    HTTP, InformationField, PositionFrame
};
