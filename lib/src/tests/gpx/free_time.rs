// GPX free_time tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gpx, read_file};


#[test]
fn test_gpx_free_time() {
    let song = read_gpx("test/free-time.gpx");
    assert!(!song.tracks.is_empty());
}

