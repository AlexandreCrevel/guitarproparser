// GP5 no_wah tests
use crate::model::song::Song;
use crate::*;
use super::super::read_file;

#[test]
fn test_gp5_no_wah() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/No Wah.gp5")))
        .unwrap();
}

