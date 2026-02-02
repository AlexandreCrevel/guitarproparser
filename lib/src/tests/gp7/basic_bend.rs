// GP7 basic_bend tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gp7, read_file};


#[test]
fn test_gp7_basic_bend() {
    let song = read_gp7("test/basic-bend.gp");
    assert!(!song.tracks.is_empty());
}

