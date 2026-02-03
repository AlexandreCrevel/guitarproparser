// GPX fingering tests
use super::super::{read_gpx};


#[test]
fn test_gpx_fingering() {
    let song = read_gpx("test/fingering.gpx");
    assert!(!song.tracks.is_empty());
}

