// GP5 fade_in tests
use crate::model::song::Song;
use super::super::read_file;

#[test]
fn test_gp5_fade_in() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/fade-in.gp5")))
        .unwrap();
}

