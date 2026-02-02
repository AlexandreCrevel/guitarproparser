// GP4 fade_in tests
use crate::model::song::Song;
use crate::*;
use super::super::read_file;

#[test]
fn test_gp4_fade_in() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/fade-in.gp4")))
        .unwrap();
}

