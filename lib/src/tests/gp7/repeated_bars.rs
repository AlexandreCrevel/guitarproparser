// GP7 repeated_bars tests
use super::super::{read_gp7};


#[test]
fn test_gp7_repeated_bars() {
    let song = read_gp7("test/repeated-bars.gp");
    assert!(!song.tracks.is_empty());
}

