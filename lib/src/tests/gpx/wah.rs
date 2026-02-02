// GPX wah tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gpx, read_file};


#[test]
fn test_gpx_wah() {
    let song = read_gpx("test/wah.gpx");
    assert!(!song.tracks.is_empty());
}

