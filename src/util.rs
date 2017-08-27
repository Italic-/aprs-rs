/// Validates an over-the-air callsign. APRS-IS is more forgiving.
///
/// Valid callsigns are between 4 and 7 alphanumeric characters with at most a single hyphen `-`
/// separating the callsign and the SSID suffix. This function validates the callsign's length,
/// composition and SSID.
///
/// # Examples
///
/// ```rust
/// # extern crate aprs;
/// # use aprs::util::valid_callsign;
/// # fn main() {
/// assert_eq!(valid_callsign("W2GMD"), true);
/// assert_eq!(valid_callsign("W2GMD-2"), true);
/// assert_eq!(valid_callsign("W2GMD*"), true);
/// # }
/// ```
///
/// ```rust
/// # extern crate aprs;
/// # use aprs::util::valid_callsign;
/// # fn main() {
/// assert_eq!(valid_callsign("W2GMD--1"), false);
/// assert_eq!(valid_callsign("W2GMD- 1"), false);
/// assert_eq!(valid_callsign("W2*MD-8"), false);
/// # }
/// ```
///
pub fn valid_callsign(callsign: &str) -> bool {
    let mut callsign: &str = callsign.trim_matches(|x| x == '*' || (x as char).is_whitespace());
    let mut ssid: &str = "0";

    // Ensure single separator between callsign and SSID
    match callsign.matches('-').count() {
        0 => {},
        1 => {
            let call_components: Vec<&str> = callsign.split('-').collect();
            callsign = call_components[0];
            ssid = call_components[1];
        },
        _ => {return false;},
    }
    // Validate callsign
    match callsign.len() {
        4...6 => {},
        _ => {return false;},
    }
    for chr in callsign.chars() {
        if !chr.is_alphanumeric() {
            return false;
        }
    }
    // Validate SSID
    match ssid.len() {
        1...2 => {},
        _ => {return false;}
    }
    for chr in ssid.chars() {
        if !chr.is_numeric() {
            return false;
        }
    }
    match ssid.parse::<usize>()  {
        Err(e) => {return false;},
        Ok(s) => {
            match s {
                0...15 => {},
                _ => {return false;},
            }
        },
    }
    // Callsign and SSID have passed all checks, so return true
    true
}
