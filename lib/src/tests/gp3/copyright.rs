// GP3 copyright tests
use crate::model::song::Song;
use crate::*;
use super::super::read_file;

#[test]
fn test_gp3_copyright() {
    let mut song: Song = Song::default();
    song.read_gp3(&read_file(String::from("test/copyright.gp3")))
        .unwrap();
}

