// GPX sforzato tests
use super::super::{read_gpx};


#[test]
fn test_gpx_sforzato() {
    let song = read_gpx("test/sforzato.gpx");
    assert!(!song.tracks.is_empty());
}

