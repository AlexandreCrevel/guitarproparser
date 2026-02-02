// GPX all_percussion tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gpx, read_file};


#[test]
fn test_gpx_all_percussion() {
    let song = read_gpx("test/all-percussion.gpx");
    assert!(!song.tracks.is_empty());
}

