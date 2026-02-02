// GPX volume_swell tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gpx, read_file};


#[test]
fn test_gpx_volume_swell() {
    let song = read_gpx("test/volume-swell.gpx");
    assert!(!song.tracks.is_empty());
}

