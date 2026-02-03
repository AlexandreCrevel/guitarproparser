// GP7 sforzato tests
use super::super::{read_gp7};


#[test]
fn test_gp7_sforzato() {
    let song = read_gp7("test/sforzato.gp");
    assert!(!song.tracks.is_empty());
}

