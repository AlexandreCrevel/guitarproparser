// GPX arpeggio tests
use super::super::{read_gpx};


#[test]
fn test_gpx_arpeggio() {
    let song = read_gpx("test/arpeggio.gpx");
    assert!(!song.tracks.is_empty());
}

