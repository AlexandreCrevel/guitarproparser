// GP7 dotted_tuplets tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gp7, read_file};


#[test]
fn test_gp7_dotted_tuplets() {
    let song = read_gp7("test/dotted-tuplets.gp");
    assert!(!song.tracks.is_empty());
}

