// GP5 vibrato tests
use crate::model::song::Song;
use super::super::read_file;

#[test]
fn test_gp5_vibrato() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/vibrato.gp5")))
        .unwrap();
}

