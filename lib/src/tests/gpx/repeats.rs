// GPX repeats tests
use super::super::{read_gpx};


#[test]
fn test_gpx_repeats() {
    let song = read_gpx("test/repeats.gpx");
    assert!(!song.measure_headers.is_empty());
    let has_repeat = song
        .measure_headers
        .iter()
        .any(|mh| mh.repeat_open || mh.repeat_close > 0);
    assert!(
        has_repeat,
        "repeats.gpx should have at least one repeat marker"
    );
}

