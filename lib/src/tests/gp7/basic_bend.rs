// GP7 basic_bend tests
use super::super::{read_gp7};


#[test]
fn test_gp7_basic_bend() {
    let song = read_gp7("test/basic-bend.gp");
    assert!(!song.tracks.is_empty());
}

