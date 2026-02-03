// GP7 mordents tests
use super::super::{read_gp7};


#[test]
fn test_gp7_mordents() {
    let song = read_gp7("test/mordents.gp");
    assert!(!song.tracks.is_empty());
}

