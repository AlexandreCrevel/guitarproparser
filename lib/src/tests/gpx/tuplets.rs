// GPX tuplets tests
use super::super::{read_gpx};


#[test]
fn test_gpx_tuplets() {
    let song = read_gpx("test/tuplets.gpx");
    assert!(!song.tracks.is_empty());
}

