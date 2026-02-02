// GPX fermata tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gpx, read_file};


#[test]
fn test_gpx_fermata() {
    let song = read_gpx("test/fermata.gpx");
    assert!(!song.tracks.is_empty());
}

