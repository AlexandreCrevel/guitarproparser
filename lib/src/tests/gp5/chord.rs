// GP5 chord tests
use crate::model::song::Song;
use super::super::read_file;

#[test]
fn test_gp5_chord() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/Chords.gp5")))
        .unwrap();
}

