//! Utilities to convert latitude and longitude into APRS-compatible strings.
//!
//! APRS latitude is a seven-character string in degree-minute-hemisphere format:
//! `DDMM.MMH`
//!
//! APRS longitude is an eight-character string in degree-minute-hemisphere format:
//! `DDDMM.MMH`
//!
//! The APRS protocol supports coordinate ambiguation by removing decimal precision. This allows an
//! operator to disclose their general location, such as city or county, without giving away
//! addresses or private information.


use decimaldegrees;


/// Convert a latitude decimal to an APRS-compatible eight-character string.
///
/// ```rust
/// # extern crate aprs;
/// # use aprs::geo_util::dec2dm_lat;
/// fn main() {
/// assert_eq!(dec2dm_lat(37.77397_f32), "3746.44N".to_string());
/// # }
/// ```
pub fn dec2dm_lat(dec: f32) -> String {
    let (deg, min): (f32, f32) = decimaldegrees::decimal2dm(dec);
    let deg_abs: f32 = deg.abs().trunc();
    let mut suffix: &str = "";
    if deg.is_sign_positive() {
        suffix = "N";
    }
    else {
        suffix = "S";
    }
    format!("{:02.0}{:05.2}{}", deg_abs, min, suffix)
}

/// Convert a longitude decimal to an APRS-compatible nine-character string.
///
/// ```rust
/// # extern crate aprs;
/// # use aprs::geo_util::dec2dm_lng;
/// fn main() {
/// assert_eq!(dec2dm_lng(-122.431297_f32), "12225.88W".to_string())
/// # }
/// ```
pub fn dec2dm_lng(dec: f32) -> String {
    let (deg, min): (f32, f32) = decimaldegrees::decimal2dm(dec);
    let deg_abs: f32 = deg.abs().trunc();
    let mut suffix: &str = "";
    if deg.is_sign_positive() {
        suffix = "E";
    }
    else {
        suffix = "W";
    }
    format!("{:03.0}{:05.2}{}", deg_abs, min, suffix)
}

/// Remove coordinate precision.
///
/// ```rust
/// # extern crate aprs;
/// # use aprs::geo_util::ambiguate;
/// # fn main() {
/// assert_eq!(ambiguate(37.77397_f32, 0), "3746.44N".to_string());
/// assert_eq!(ambiguate(37.77397_f32, 1), "3746.4 N".to_string());
/// assert_eq!(ambiguate(37.77397_f32, 3), "374 .  N".to_string());
/// # }
/// ```
// pub fn ambiguate(pos: f32, ambiguity: u8) -> String {
// }
