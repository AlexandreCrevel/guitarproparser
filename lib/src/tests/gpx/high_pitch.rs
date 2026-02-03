// GPX high_pitch tests
use super::super::{read_gpx};


#[test]
fn test_gpx_high_pitch() {
    let song = read_gpx("test/high-pitch.gpx");
    assert!(!song.tracks.is_empty());
}

