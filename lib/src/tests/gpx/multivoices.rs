// GPX multivoices tests
use super::super::{read_gpx};


#[test]
fn test_gpx_multivoices() {
    let song = read_gpx("test/multivoices.gpx");
    assert!(!song.tracks.is_empty());
}

