// GPX timer tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gpx, read_file};


#[test]
fn test_gpx_timer() {
    let song = read_gpx("test/timer.gpx");
    assert!(!song.tracks.is_empty());
}

