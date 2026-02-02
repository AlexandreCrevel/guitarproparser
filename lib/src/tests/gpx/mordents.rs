// GPX mordents tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gpx, read_file};


#[test]
fn test_gpx_mordents() {
    let song = read_gpx("test/mordents.gpx");
    assert!(!song.tracks.is_empty());
}

