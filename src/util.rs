//! Utilities for validating data.

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
        3...6 => {},
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


/// Hashing algorithm to generate a passcode for APRS-IS.
/// Original code by Steve Dimse in the [aprsd package][1].
///
/// ```rust
/// # extern crate aprs;
/// # use aprs::util::aprspass;
/// # fn main() {
/// // Callsigns can be uppercase...
/// assert_eq!(aprspass("W2GMD"), Some(10141));
/// // ...or lowercase
/// assert_eq!(aprspass("n6gso"), Some(13703));
/// # }
/// ```
///
/// ```text
/// short doHash(const char *theCall) {
///     char rootCall[10];            // need to copy call to remove ssid from parse
///     char *p1 = rootCall;
///
///     while ((*theCall != '-') && (*theCall != 0)) *p1++ = toupper(*theCall++);
///     *p1 = 0;
///
///     short hash = 0x73e2;          // Initialize with the key value
///     short i = 0;
///     short len = strlen(rootCall);
///     char *ptr = rootCall;
///
///     while (i<len) {               // Loop through the string two bytes at a time
///         hash ^= (*ptr++)<<8;      // xor high byte with accumulated hash
///         hash ^= (*ptr++);         // xor low byte with accumulated hash
///         i += 2;
///     }
///     return hash & 0x7fff;         // mask off the high bit so number is always positive
/// }
/// ```
///
/// [1]: ftp://ftp.tapr.org/software_lib/Linux/aprsd/
pub fn aprspass(callsign: &str) -> Option<u16> {
    match valid_callsign(callsign) {
        false => return None,
        true => (),
    }
    let mut hash: u16 = 0x73e2;

    for chunk in callsign.to_uppercase().bytes().collect::<Vec<u8>>().chunks(2) {
        match chunk.len() {
            1 => {
                hash = hash ^ ((chunk[0] as u16) << 8);
            },
            2 => {
                hash = hash ^ ((chunk[0] as u16) << 8);
                hash = hash ^ (chunk[1] as u16);
            },
            _ => {},
        }
    }
    Some(hash & 0x7fff)
}
