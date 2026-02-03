// GP5 strokes tests
use crate::model::song::Song;
use super::super::read_file;

#[test]
fn test_gp5_strokes() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/Strokes.gp5")))
        .unwrap();
}

