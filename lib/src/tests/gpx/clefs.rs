// GPX clefs tests
use super::super::{read_gpx};


#[test]
fn test_gpx_clefs() {
    let song = read_gpx("test/clefs.gpx");
    assert!(!song.tracks.is_empty());
}

