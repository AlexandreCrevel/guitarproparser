// GP7 pick_up_down tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gp7, read_file};


#[test]
fn test_gp7_pick_up_down() {
    let song = read_gp7("test/pick-up-down.gp");
    assert!(!song.tracks.is_empty());
}

