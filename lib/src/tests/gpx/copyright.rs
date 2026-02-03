// GPX copyright tests
use super::super::{read_gpx};


#[test]
fn test_gpx_copyright() {
    let song = read_gpx("test/copyright.gpx");
    assert!(!song.tracks.is_empty());
    assert!(
        !song.copyright.is_empty(),
        "copyright field should be populated"
    );
}

