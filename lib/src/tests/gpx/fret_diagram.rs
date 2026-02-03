// GPX fret_diagram tests
use super::super::{read_gpx};


#[test]
fn test_gpx_fret_diagram() {
    let song = read_gpx("test/fret-diagram.gpx");
    assert!(!song.tracks.is_empty());
}

