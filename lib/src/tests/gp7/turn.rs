// GP7 turn tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gp7, read_file};


#[test]
fn test_gp7_turn() {
    let song = read_gp7("test/turn.gp");
    assert!(!song.tracks.is_empty());
}

