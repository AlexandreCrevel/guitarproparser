// GP7 fret_diagram tests
use super::super::{read_gp7};


#[test]
fn test_gp7_fret_diagram() {
    let song = read_gp7("test/fret-diagram.gp");
    assert!(!song.tracks.is_empty());
}

