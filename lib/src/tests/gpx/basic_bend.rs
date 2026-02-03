// GPX basic_bend tests
use super::super::{read_gpx};


#[test]
fn test_gpx_basic_bend() {
    let song = read_gpx("test/basic-bend.gpx");
    assert!(!song.tracks.is_empty());
}

