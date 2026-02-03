// GP7 barre tests
use super::super::{read_gp7};


#[test]
fn test_gp7_barre() {
    let song = read_gp7("test/barre.gp");
    assert!(!song.tracks.is_empty());
}

