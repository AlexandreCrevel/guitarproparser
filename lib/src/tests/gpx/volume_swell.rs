// GPX volume_swell tests
use super::super::{read_gpx};


#[test]
fn test_gpx_volume_swell() {
    let song = read_gpx("test/volume-swell.gpx");
    assert!(!song.tracks.is_empty());
}

