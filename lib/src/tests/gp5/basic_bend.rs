// GP5 basic_bend tests
use crate::model::song::Song;
use super::super::read_file;

#[test]
fn test_gp5_basic_bend() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/basic-bend.gp5")))
        .unwrap();
}

