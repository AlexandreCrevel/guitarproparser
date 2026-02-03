// GPX keysig tests
use super::super::{read_gpx};


#[test]
fn test_gpx_keysig() {
    let song = read_gpx("test/keysig.gpx");
    assert_eq!(song.tracks.len(), 1);
    assert_eq!(song.measure_headers.len(), 32);
}

