// GP7 rasg tests
use super::super::{read_gp7};


#[test]
fn test_gp7_rasg() {
    let song = read_gp7("test/rasg.gp");
    assert!(!song.tracks.is_empty());
}

