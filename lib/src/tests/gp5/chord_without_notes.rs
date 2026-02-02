// GP5 chord_without_notes tests
use crate::model::song::Song;
use crate::*;
use super::super::read_file;

#[test]
fn test_gp5_chord_without_notes() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/chord_without_notes.gp5")))
        .unwrap();
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/001_Funky_Guy.gp5")))
        .unwrap();
}

