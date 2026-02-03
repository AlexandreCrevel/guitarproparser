// GP7 chordnames_keyboard tests
use super::super::{read_gp7};


#[test]
fn test_gp7_chordnames_keyboard() {
    let song = read_gp7("test/chordnames_keyboard.gp");
    assert!(!song.tracks.is_empty());
}

