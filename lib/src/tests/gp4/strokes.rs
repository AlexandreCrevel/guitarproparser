// GP4 strokes tests
use crate::model::song::Song;
use super::super::read_file;

#[test]
fn test_gp4_strokes() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/Strokes.gp4")))
        .unwrap();
}

