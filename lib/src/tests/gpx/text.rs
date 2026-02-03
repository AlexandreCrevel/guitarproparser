// GPX text tests
use super::super::{read_gpx};


#[test]
fn test_gpx_text() {
    let song = read_gpx("test/text.gpx");
    assert!(!song.tracks.is_empty());
}

