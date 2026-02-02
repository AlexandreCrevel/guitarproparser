// GPX fret_diagram tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gpx, read_file};


#[test]
fn test_gpx_fret_diagram() {
    let song = read_gpx("test/fret-diagram.gpx");
    assert!(!song.tracks.is_empty());
}

