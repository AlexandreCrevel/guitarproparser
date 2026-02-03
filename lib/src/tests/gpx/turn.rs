// GPX turn tests
use super::super::{read_gpx};


#[test]
fn test_gpx_turn() {
    let song = read_gpx("test/turn.gpx");
    assert!(!song.tracks.is_empty());
}

