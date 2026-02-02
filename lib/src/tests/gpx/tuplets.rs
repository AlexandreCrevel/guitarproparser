// GPX tuplets tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gpx, read_file};


#[test]
fn test_gpx_tuplets() {
    let song = read_gpx("test/tuplets.gpx");
    assert!(!song.tracks.is_empty());
}

