extern crate aprs;
use aprs::geo_util;

#[test]
fn test_dec2dm_lat() {
    assert_eq!(geo_util::dec2dm_lat(37.77397_f32), "3746.44N".to_string());
}

#[test]
fn test_dec2dm_lng() {
    assert_eq!(geo_util::dec2dm_lng(-122.431297_f32), "12225.88W".to_string())
}

// #[test]
// #[ignore]
// fn test_ambiguate() {
//     assert_eq!(geo_util::ambiguate(), );
// }
