// GPX all_percussion tests
use super::super::{read_gpx};


#[test]
fn test_gpx_all_percussion() {
    let song = read_gpx("test/all-percussion.gpx");
    assert!(!song.tracks.is_empty());
}

