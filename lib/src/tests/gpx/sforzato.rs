// GPX sforzato tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gpx, read_file};


#[test]
fn test_gpx_sforzato() {
    let song = read_gpx("test/sforzato.gpx");
    assert!(!song.tracks.is_empty());
}

