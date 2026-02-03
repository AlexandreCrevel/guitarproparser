// GP7 turn tests
use super::super::{read_gp7};


#[test]
fn test_gp7_turn() {
    let song = read_gp7("test/turn.gp");
    assert!(!song.tracks.is_empty());
}

