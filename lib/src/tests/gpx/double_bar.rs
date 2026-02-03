// GPX double_bar tests
use super::super::{read_gpx};


#[test]
fn test_gpx_double_bar() {
    let song = read_gpx("test/double-bar.gpx");
    assert!(!song.measure_headers.is_empty());
    let has_double_bar = song.measure_headers.iter().any(|mh| mh.double_bar);
    assert!(
        has_double_bar,
        "double-bar.gpx should have at least one double bar"
    );
}

