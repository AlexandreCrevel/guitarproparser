// GP5 rse tests
use crate::model::song::Song;
use crate::*;
use super::super::read_file;

#[test]
fn test_gp5_rse() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/RSE.gp5")))
        .unwrap();
}

