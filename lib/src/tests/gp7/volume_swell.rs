// GP7 volume_swell tests
use super::super::{read_gp7};


#[test]
fn test_gp7_volume_swell() {
    let song = read_gp7("test/volume-swell.gp");
    assert!(!song.tracks.is_empty());
}

