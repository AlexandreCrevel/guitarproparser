// GPX free_time tests
use super::super::{read_gpx};


#[test]
fn test_gpx_free_time() {
    let song = read_gpx("test/free-time.gpx");
    assert!(!song.tracks.is_empty());
}

