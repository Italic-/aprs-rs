//! This crate provides connectivity to the APRS amateur radio network over the internet.
//!
//! APRS (Automatic Packet Reporting System) is a packet-based, internet- and rf-connected
//! communication network for amateur radio operators and enthusiasts that allows transport
//! of location data, weather reports and more among operators. See the [APRS website][1]
//! for a detailed description of the network.
//!
//! Credits:
//! --------
//! APRS (c) Bob Bruninga, WB4APR  
//! [Original Python module][2] by Greg Albrecht, W2GMD  
//! Rust port by Jeffrey Hoover, KI6SGV  
//!
//! This library is licensed under the Apache 2.0 license. See [LICENSE][2] for full terms.
//!
//! [1]: http://www.aprs.org/
//! [2]: https://www.github.com/ampledata/aprs
//! [3]: https://www.apache.org/licenses/LICENSE-2.0

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

pub mod constants;
pub mod util;
pub mod geo_util;
pub mod fcs;
pub mod functions;
pub mod structs;
pub mod decimaldegrees;
pub mod kiss_structs;
