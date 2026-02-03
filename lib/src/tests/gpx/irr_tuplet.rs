// GPX irr_tuplet tests
use super::super::{read_gpx};


#[test]
fn test_gpx_test_irr_tuplet() {
    let song = read_gpx("test/testIrrTuplet.gpx");
    assert!(!song.tracks.is_empty());
}

