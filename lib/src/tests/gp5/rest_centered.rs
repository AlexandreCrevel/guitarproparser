// GP5 rest_centered tests
use crate::model::song::Song;
use crate::*;
use super::super::read_file;

#[test]
fn test_gp5_rest_centered() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/rest-centered.gp5")))
        .unwrap();
}

