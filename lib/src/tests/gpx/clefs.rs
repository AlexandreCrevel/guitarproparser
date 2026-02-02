// GPX clefs tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gpx, read_file};


#[test]
fn test_gpx_clefs() {
    let song = read_gpx("test/clefs.gpx");
    assert!(!song.tracks.is_empty());
}

