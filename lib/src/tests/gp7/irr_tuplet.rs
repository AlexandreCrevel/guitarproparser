// GP7 irr_tuplet tests
use super::super::{read_gp7};


#[test]
fn test_gp7_test_irr_tuplet() {
    let song = read_gp7("test/testIrrTuplet.gp");
    assert!(!song.tracks.is_empty());
}

