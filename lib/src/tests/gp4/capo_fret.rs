// GP4 capo_fret tests
use crate::model::song::Song;
use crate::*;
use super::super::read_file;

#[test]
fn test_gp4_capo_fret() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/capo-fret.gp4")))
        .unwrap();
}

