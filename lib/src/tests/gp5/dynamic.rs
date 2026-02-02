// GP5 dynamic tests
use crate::model::song::Song;
use crate::*;
use super::super::read_file;

#[test]
fn test_gp5_dynamic() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/dynamic.gp5")))
        .unwrap();
}

