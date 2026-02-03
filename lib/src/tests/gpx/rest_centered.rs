// GPX rest_centered tests
use super::super::{read_gpx};


#[test]
fn test_gpx_rest_centered() {
    let song = read_gpx("test/rest-centered.gpx");
    assert!(!song.tracks.is_empty());
}

