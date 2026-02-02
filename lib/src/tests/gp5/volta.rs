// GP5 volta tests
use crate::model::song::Song;
use crate::*;
use super::super::read_file;

#[test]
fn test_gp5_volta() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/volta.gp5")))
        .unwrap();
}

