// GP7 text tests
use super::super::{read_gp7};


#[test]
fn test_gp7_text() {
    let song = read_gp7("test/text.gp");
    assert!(!song.tracks.is_empty());
}

