// GP7 tremolos tests
use super::super::{read_gp7};


#[test]
fn test_gp7_tremolos() {
    let song = read_gp7("test/tremolos.gp");
    assert!(!song.tracks.is_empty());
}

