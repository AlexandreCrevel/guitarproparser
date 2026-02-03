// GP7 crescendo_diminuendo tests
use super::super::{read_gp7};


#[test]
fn test_gp7_crescendo_diminuendo() {
    let song = read_gp7("test/crescendo-diminuendo.gp");
    assert!(!song.tracks.is_empty());
}

