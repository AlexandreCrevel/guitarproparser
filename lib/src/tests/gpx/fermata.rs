// GPX fermata tests
use super::super::{read_gpx};


#[test]
fn test_gpx_fermata() {
    let song = read_gpx("test/fermata.gpx");
    assert!(!song.tracks.is_empty());
}

