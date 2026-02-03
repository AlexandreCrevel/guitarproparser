// GP5 voices tests
use crate::model::song::Song;
use super::super::read_file;

#[test]
fn test_gp5_voices() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/Voices.gp5")))
        .unwrap();
}

