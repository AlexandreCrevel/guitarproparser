// GPX dotted_tuplets tests
use super::super::{read_gpx};


#[test]
fn test_gpx_dotted_tuplets() {
    let song = read_gpx("test/dotted-tuplets.gpx");
    assert!(!song.tracks.is_empty());
}

