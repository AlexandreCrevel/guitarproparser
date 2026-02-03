// GP7 heavy_accent tests
use super::super::{read_gp7};


#[test]
fn test_gp7_heavy_accent() {
    let song = read_gp7("test/heavy-accent.gp");
    assert!(!song.tracks.is_empty());
}

