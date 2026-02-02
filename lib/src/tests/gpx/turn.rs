// GPX turn tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gpx, read_file};


#[test]
fn test_gpx_turn() {
    let song = read_gpx("test/turn.gpx");
    assert!(!song.tracks.is_empty());
}

