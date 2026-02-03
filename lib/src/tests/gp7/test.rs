// GP7 test tests
use super::super::{read_gp7};


#[test]
fn test_gp7_test() {
    let song = read_gp7("test/test.gp");
    assert!(!song.tracks.is_empty());
}

