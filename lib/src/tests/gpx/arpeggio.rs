// GPX arpeggio tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gpx, read_file};


#[test]
fn test_gpx_arpeggio() {
    let song = read_gpx("test/arpeggio.gpx");
    assert!(!song.tracks.is_empty());
}

