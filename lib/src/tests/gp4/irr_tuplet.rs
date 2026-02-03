// GP4 irr_tuplet tests
use crate::model::song::Song;
use super::super::read_file;

#[test]
fn test_gp4_test_irr_tuplet() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/testIrrTuplet.gp4")))
        .unwrap();
}

