// GP5 keysig tests
use crate::model::song::Song;
use super::super::read_file;

#[test]
fn test_gp5_keysig() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/keysig.gp5")))
        .unwrap();
}

