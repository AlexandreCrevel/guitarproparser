// GPX repeated_bars tests
use super::super::{read_gpx};


#[test]
fn test_gpx_repeated_bars() {
    let song = read_gpx("test/repeated-bars.gpx");
    assert!(!song.tracks.is_empty());
}

