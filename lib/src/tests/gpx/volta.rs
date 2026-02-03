// GPX volta tests
use super::super::{read_gpx};


#[test]
fn test_gpx_volta() {
    let song = read_gpx("test/volta.gpx");
    assert!(!song.measure_headers.is_empty());
    let has_volta = song
        .measure_headers
        .iter()
        .any(|mh| mh.repeat_alternative > 0);
    assert!(
        has_volta,
        "volta.gpx should have at least one alternate ending"
    );
}

