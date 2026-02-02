// GPX tremolos tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gpx, read_file};


#[test]
fn test_gpx_tremolos() {
    let song = read_gpx("test/tremolos.gpx");
    assert!(!song.tracks.is_empty());
}

