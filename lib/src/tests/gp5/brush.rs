// GP5 brush tests
use crate::model::song::Song;
use super::super::read_file;

#[test]
fn test_gp5_brush() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/brush.gp5")))
        .unwrap();
}

