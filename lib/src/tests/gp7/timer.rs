// GP7 timer tests
use super::super::{read_gp7};


#[test]
fn test_gp7_timer() {
    let song = read_gp7("test/timer.gp");
    assert!(!song.tracks.is_empty());
}

