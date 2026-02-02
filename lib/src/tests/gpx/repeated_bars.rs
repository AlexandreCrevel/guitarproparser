// GPX repeated_bars tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gpx, read_file};


#[test]
fn test_gpx_repeated_bars() {
    let song = read_gpx("test/repeated-bars.gpx");
    assert!(!song.tracks.is_empty());
}

