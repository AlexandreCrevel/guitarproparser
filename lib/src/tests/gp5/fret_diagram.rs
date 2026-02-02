// GP5 fret_diagram tests
use crate::model::song::Song;
use crate::*;
use super::super::read_file;

#[test]
fn test_gp5_fret_diagram() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/fret-diagram.gp5")))
        .unwrap();
}

