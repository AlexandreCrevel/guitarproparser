// GPX directions tests
use super::super::{read_gpx};


#[test]
fn test_gpx_directions() {
    let song = read_gpx("test/directions.gpx");
    assert!(!song.measure_headers.is_empty());
}

