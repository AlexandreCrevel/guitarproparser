// GP3 chord tests
use crate::model::song::Song;
use super::super::read_file;

#[test]
fn test_gp3_chord() {
    let mut song: Song = Song::default();
    song.read_gp3(&read_file(String::from("test/Chords.gp3")))
        .unwrap();
}

