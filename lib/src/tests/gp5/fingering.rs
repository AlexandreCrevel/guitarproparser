// GP5 fingering tests
use crate::model::song::Song;
use crate::*;
use super::super::read_file;

#[test]
fn test_gp5_fingering() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/fingering.gp5")))
        .unwrap();
}

