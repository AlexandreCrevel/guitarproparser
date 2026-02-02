// GPX rest_centered tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gpx, read_file};


#[test]
fn test_gpx_rest_centered() {
    let song = read_gpx("test/rest-centered.gpx");
    assert!(!song.tracks.is_empty());
}

