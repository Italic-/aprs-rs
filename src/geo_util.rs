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
    if deg.is_sign_positive() {
        format!("{:02.0}{:05.2}N", deg_abs, min)
    }
    else {
        format!("{:02.0}{:05.2}S", deg_abs, min)
    }
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
    if deg.is_sign_positive() {
        format!("{:03.0}{:05.2}E", deg_abs, min)
    }
    else {
        format!("{:03.0}{:05.2}W", deg_abs, min)
    }
}

/// Remove coordinate precision.
///
/// ```rust
/// # extern crate aprs;
/// # use aprs::geo_util::ambiguate;
/// # fn main() {
/// assert_eq!(ambiguate("12225.88W", 0), "12225.88W".to_string());
/// assert_eq!(ambiguate("12225.88W", 3), "1222 .  W".to_string());
/// assert_eq!(ambiguate("3746.44N", 1), "3746.4 N".to_string());
/// # }
/// ```
pub fn ambiguate(pos: &str, ambiguity: usize) -> String {
    let mut amb: usize = ambiguity;
    match ambiguity {
        0 => {return pos.to_string();},
        1...2 => {},
        _ => {amb += 1;}
    }

    let start: usize = pos.len() - (amb + 1);
    let end: usize = pos.len() - 1;

    let mut pos_ambig: String = String::new();

    for (ind, chr) in pos.char_indices() {
        if ind >= start && ind < end {
            match chr {
                '.' => {pos_ambig.push(chr);},
                _ => {pos_ambig.push(' ');},
            }
        } else {
            pos_ambig.push(chr);
        }
    }
    pos_ambig
}
