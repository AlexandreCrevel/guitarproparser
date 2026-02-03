// GPX brush tests
use super::super::{read_gpx};


#[test]
fn test_gpx_brush() {
    let song = read_gpx("test/brush.gpx");
    assert!(!song.tracks.is_empty());
}

