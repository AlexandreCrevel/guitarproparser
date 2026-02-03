// GP4 slurs tests
use crate::model::song::Song;
use super::super::read_file;

#[test]
fn test_gp4_slur() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/slur.gp4")))
        .unwrap();
}

#[test]
fn test_gp4_tuplet_with_slur() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/tuplet-with-slur.gp4")))
        .unwrap();
}

