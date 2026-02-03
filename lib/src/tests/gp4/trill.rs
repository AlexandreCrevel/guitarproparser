// GP4 trill tests
use crate::model::song::Song;
use super::super::read_file;

#[test]
fn test_gp4_trill() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/trill.gp4")))
        .unwrap();
}

