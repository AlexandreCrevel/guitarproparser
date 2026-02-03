// GPX tremolos tests
use super::super::{read_gpx};


#[test]
fn test_gpx_tremolos() {
    let song = read_gpx("test/tremolos.gpx");
    assert!(!song.tracks.is_empty());
}

