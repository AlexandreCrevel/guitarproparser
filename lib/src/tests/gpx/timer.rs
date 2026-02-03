// GPX timer tests
use super::super::{read_gpx};


#[test]
fn test_gpx_timer() {
    let song = read_gpx("test/timer.gpx");
    assert!(!song.tracks.is_empty());
}

