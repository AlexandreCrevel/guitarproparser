// GPX fingering tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gpx, read_file};


#[test]
fn test_gpx_fingering() {
    let song = read_gpx("test/fingering.gpx");
    assert!(!song.tracks.is_empty());
}

