// GP7 all_percussion tests
use super::super::{read_gp7};


#[test]
fn test_gp7_all_percussion() {
    let song = read_gp7("test/all-percussion.gp");
    assert!(!song.tracks.is_empty());
}

