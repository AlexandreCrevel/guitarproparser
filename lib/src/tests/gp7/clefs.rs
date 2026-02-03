// GP7 clefs tests
use super::super::{read_gp7};


#[test]
fn test_gp7_clefs() {
    let song = read_gp7("test/clefs.gp");
    assert!(!song.tracks.is_empty());
}

