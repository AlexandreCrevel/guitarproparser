// GPX accent tests
use super::super::{read_gpx};


#[test]
fn test_gpx_accent() {
    let song = read_gpx("test/accent.gpx");
    assert!(!song.tracks.is_empty());
}

