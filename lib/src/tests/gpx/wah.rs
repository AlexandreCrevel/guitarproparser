// GPX wah tests
use super::super::{read_gpx};


#[test]
fn test_gpx_wah() {
    let song = read_gpx("test/wah.gpx");
    assert!(!song.tracks.is_empty());
}

