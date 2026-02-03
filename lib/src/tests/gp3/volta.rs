// GP3 volta tests
use crate::model::song::Song;
use super::super::read_file;

#[test]
fn test_gp3_volta() {
    let mut song: Song = Song::default();
    song.read_gp3(&read_file(String::from("test/volta.gp3")))
        .unwrap();
}

