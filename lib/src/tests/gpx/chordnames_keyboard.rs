// GPX chordnames_keyboard tests
use super::super::{read_gpx};


#[test]
fn test_gpx_chordnames_keyboard() {
    let song = read_gpx("test/chordnames_keyboard.gpx");
    assert!(!song.tracks.is_empty());
}

