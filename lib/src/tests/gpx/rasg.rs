// GPX rasg tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gpx, read_file};


#[test]
fn test_gpx_rasg() {
    let song = read_gpx("test/rasg.gpx");
    assert!(!song.tracks.is_empty());
}

