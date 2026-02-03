// GPX mordents tests
use super::super::{read_gpx};


#[test]
fn test_gpx_mordents() {
    let song = read_gpx("test/mordents.gpx");
    assert!(!song.tracks.is_empty());
}

