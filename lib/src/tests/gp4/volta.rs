// GP4 volta tests
use crate::model::song::Song;
use super::super::read_file;

#[test]
fn test_gp4_volta() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/volta.gp4")))
        .unwrap();
}

