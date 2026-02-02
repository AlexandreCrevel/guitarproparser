// GP5 copyright tests
use crate::model::song::Song;
use crate::*;
use super::super::read_file;

#[test]
fn test_gp5_copyright() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/copyright.gp5")))
        .unwrap();
}

