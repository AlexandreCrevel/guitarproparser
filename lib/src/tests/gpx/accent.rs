// GPX accent tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gpx, read_file};


#[test]
fn test_gpx_accent() {
    let song = read_gpx("test/accent.gpx");
    assert!(!song.tracks.is_empty());
}

