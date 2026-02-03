// GP7 high_pitch tests
use super::super::{read_gp7};


#[test]
fn test_gp7_high_pitch() {
    let song = read_gp7("test/high-pitch.gp");
    assert!(!song.tracks.is_empty());
}

