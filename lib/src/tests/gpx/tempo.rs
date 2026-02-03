// GPX tempo tests
use super::super::{read_gpx};


#[test]
fn test_gpx_tempo() {
    let song = read_gpx("test/tempo.gpx");
    assert!(!song.measure_headers.is_empty());
    assert!(song.tempo > 0, "tempo should be parsed from automations");
}

