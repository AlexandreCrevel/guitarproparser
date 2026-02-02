// GP3 effects tests
use crate::model::song::Song;
use crate::*;
use super::super::read_file;

#[test]
fn test_gp3_effects() {
    let mut song: Song = Song::default();
    song.read_gp3(&read_file(String::from("test/Effects.gp3")))
        .unwrap();
}

