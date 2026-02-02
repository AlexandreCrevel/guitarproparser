// GP5 all_percussion tests
use crate::model::song::Song;
use crate::*;
use super::super::read_file;

#[test]
fn test_gp5_all_percussion() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/all-percussion.gp5")))
        .unwrap();
}

