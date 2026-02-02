// GP7 text tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gp7, read_file};


#[test]
fn test_gp7_text() {
    let song = read_gp7("test/text.gp");
    assert!(!song.tracks.is_empty());
}

