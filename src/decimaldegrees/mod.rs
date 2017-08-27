//! This module provides utilities to convert between various coordinate notation styles.
//!
//! Latitude and longitude may be translated from DM(S) (degrees, minutes, seconds (optionally)) to floating
//! values and vice versa.
//!
//! For copyright information, see [LICENSE](LICENSE).

/// Translate a floating point angle measure into degrees, minutes and seconds.
///
/// ```rust
/// # extern crate aprs;
/// # use aprs::decimaldegrees::decimal2dms;
/// # fn main() {
/// assert_eq!(decimal2dms(37.773972_f32), (37_f32, 46_f32, 26.297607_f32));
/// assert_eq!(decimal2dms(-122.43111_f32), (-122_f32, 25_f32, 51.983643_f32));
/// # }
/// ```
pub fn decimal2dms(decimal_degrees: f32) -> (f32, f32, f32) {
    let degrees: f32 = decimal_degrees.trunc();
    let dec_min: f32 = (decimal_degrees - degrees).abs() * 60_f32;
    let minutes: f32 = dec_min.abs().trunc();
    let seconds: f32 = (dec_min - minutes) * 60_f32;
    (degrees, minutes, seconds)
}


/// Translate a floating point angle measure into degrees and minutes.
///
/// ```rust
/// # extern crate aprs;
/// # use aprs::decimaldegrees::decimal2dm;
/// # fn main() {
/// assert_eq!(decimal2dm(37.773888_f32), (37_f32, 46.433258_f32));
/// assert_eq!(decimal2dm(-122.43111_f32), (-122_f32, 25.866394_f32));
/// # }
/// ```
pub fn decimal2dm(decimal_degrees: f32) -> (f32, f32) {
    let degrees: f32 = decimal_degrees.trunc();
    let minutes: f32 = (decimal_degrees - degrees).abs() * 60_f32;
    (degrees, minutes)
}


/// Translate a DMS angle measure into a floating point degree measure.
///
/// ```rust
/// # extern crate aprs;
/// # use aprs::decimaldegrees::dms2decimal;
/// # fn main() {
/// assert_eq!(dms2decimal(37_f32, 46_f32, 26_f32), 37.773888_f32);
/// assert_eq!(dms2decimal(-122_f32, 25_f32, 52_f32), -122.43111_f32);
/// # }
/// ```
pub fn dms2decimal(degrees: f32, minutes: f32, seconds: f32) -> f32 {
    let degs: f32 = degrees.trunc();
    let mins: f32 = minutes / 60_f32;
    let secs: f32 = seconds / 3600_f32;
    if degrees.is_sign_positive() {
        degs + mins + secs
    }
    else {
        degs - mins - secs
    }
}

/// Translate a DM angle measure into a floating point degree measure.
///
/// This function takes degrees and minutes and returns a float similar to `dms2decimal()`.
///
/// ```rust
/// # extern crate aprs;
/// # use aprs::decimaldegrees::dm2decimal;
/// # fn main() {
/// assert_eq!(dm2decimal(37_f32, 46.43832_f32), 37.773972_f32);
/// assert_eq!(dm2decimal(-122_f32, 25.877838_f32), -122.431297_f32);
/// # }
/// ```
pub fn dm2decimal(degrees: f32, minutes: f32) -> f32 {
    dms2decimal(degrees, minutes, 0_f32)
}
