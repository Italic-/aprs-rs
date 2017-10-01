extern crate aprs;
use aprs::util::aprspass;

#[test]
fn test_pass() {
    assert_eq!(aprspass("WA6SYN"), Some(19390));
    assert_eq!(aprspass("n6gso"), Some(13703));
}
