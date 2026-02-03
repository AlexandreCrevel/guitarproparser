// GPX heavy_accent tests
use super::super::{read_gpx};


#[test]
fn test_gpx_heavy_accent() {
    let song = read_gpx("test/heavy-accent.gpx");
    assert!(!song.tracks.is_empty());
}

