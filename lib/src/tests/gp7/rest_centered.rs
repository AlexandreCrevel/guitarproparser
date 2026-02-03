// GP7 rest_centered tests
use super::super::{read_gp7};


#[test]
fn test_gp7_rest_centered() {
    let song = read_gp7("test/rest-centered.gp");
    assert!(!song.tracks.is_empty());
}

