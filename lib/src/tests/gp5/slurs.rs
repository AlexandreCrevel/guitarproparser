// GP5 slurs tests
use crate::model::song::Song;
use crate::*;
use super::super::read_file;

#[test]
fn test_gp5_slur_notes_effect_mask() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/slur-notes-effect-mask.gp5")))
        .unwrap();
}

