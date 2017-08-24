extern crate aprs;
use aprs::util;

static VALID_CALLSIGNS: &'static [&'static str; 7] = &[
    "W2GMD", "W2GMD-1", "KF4MKT", "KF4MKT-1",
    "KF4LZA-15", "W2GMD*", "OTTFFS*"
];
static INVALID_CALLSIGNS: &'static [&'static str; 8] = &[
    "xW2GMDx", "W2GMD-16", "W2GMD-A", "W", "W2GMD-1-0",
    "W*GMD", "W2GMD-123", "W2GMD-123*"
];

#[test]
fn valid_callsign() {
    for sign in VALID_CALLSIGNS {
        assert_eq!(util::valid_callsign(sign), true);
    }
}

#[test]
fn invalid_callsign() {
    for sign in INVALID_CALLSIGNS {
        assert_eq!(util::valid_callsign(sign), false);
    }
}
