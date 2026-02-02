// GP4 key tests
use crate::model::song::Song;
use crate::*;
use super::super::read_file;

#[test]
fn test_gp4_key() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/Key.gp4")))
        .unwrap();
}

