// GP4 copyright tests
use crate::model::song::Song;
use crate::*;
use super::super::read_file;

#[test]
fn test_gp4_copyright() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/copyright.gp4")))
        .unwrap();
}

