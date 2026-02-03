// GP7 brush tests
use super::super::{read_gp7};


#[test]
fn test_gp7_brush() {
    let song = read_gp7("test/brush.gp");
    assert!(!song.tracks.is_empty());
}

