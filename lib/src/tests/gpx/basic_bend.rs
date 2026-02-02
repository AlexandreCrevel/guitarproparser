// GPX basic_bend tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gpx, read_file};


#[test]
fn test_gpx_basic_bend() {
    let song = read_gpx("test/basic-bend.gpx");
    assert!(!song.tracks.is_empty());
}

