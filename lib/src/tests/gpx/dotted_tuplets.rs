// GPX dotted_tuplets tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gpx, read_file};


#[test]
fn test_gpx_dotted_tuplets() {
    let song = read_gpx("test/dotted-tuplets.gpx");
    assert!(!song.tracks.is_empty());
}

