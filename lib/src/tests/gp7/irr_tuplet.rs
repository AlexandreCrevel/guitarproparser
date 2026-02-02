// GP7 irr_tuplet tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gp7, read_file};


#[test]
fn test_gp7_test_irr_tuplet() {
    let song = read_gp7("test/testIrrTuplet.gp");
    assert!(!song.tracks.is_empty());
}

