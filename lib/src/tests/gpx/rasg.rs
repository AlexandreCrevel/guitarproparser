// GPX rasg tests
use super::super::{read_gpx};


#[test]
fn test_gpx_rasg() {
    let song = read_gpx("test/rasg.gpx");
    assert!(!song.tracks.is_empty());
}

