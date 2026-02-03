// GPX pick_up_down tests
use super::super::{read_gpx};


#[test]
fn test_gpx_pick_up_down() {
    let song = read_gpx("test/pick-up-down.gpx");
    assert!(!song.tracks.is_empty());
}

