// GP7 wah tests
use super::super::{read_gp7};


#[test]
fn test_gp7_wah() {
    let song = read_gp7("test/wah.gp");
    assert!(!song.tracks.is_empty());
}

