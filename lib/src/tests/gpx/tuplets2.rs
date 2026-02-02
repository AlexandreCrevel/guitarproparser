// GPX tuplets2 tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gpx, read_file};


#[test]
fn test_gpx_tuplets2() {
    let song = read_gpx("test/tuplets2.gpx");
    assert!(!song.tracks.is_empty());
}

