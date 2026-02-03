// GPX barre tests
use super::super::{read_gpx};


#[test]
fn test_gpx_barre() {
    let song = read_gpx("test/barre.gpx");
    assert!(!song.tracks.is_empty());
}

