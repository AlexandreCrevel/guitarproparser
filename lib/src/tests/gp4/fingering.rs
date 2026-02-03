// GP4 fingering tests
use crate::model::song::Song;
use super::super::read_file;

#[test]
fn test_gp4_fingering() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/fingering.gp4")))
        .unwrap();
}

