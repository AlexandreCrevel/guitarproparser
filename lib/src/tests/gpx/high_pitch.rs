// GPX high_pitch tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gpx, read_file};


#[test]
fn test_gpx_high_pitch() {
    let song = read_gpx("test/high-pitch.gpx");
    assert!(!song.tracks.is_empty());
}

