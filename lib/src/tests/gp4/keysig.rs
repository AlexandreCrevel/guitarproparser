// GP4 keysig tests
use crate::model::song::Song;
use super::super::read_file;

#[test]
fn test_gp4_keysig() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/keysig.gp4")))
        .unwrap();
}

