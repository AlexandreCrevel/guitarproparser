// GP5 pick_up_down tests
use crate::model::song::Song;
use crate::*;
use super::super::read_file;

#[test]
fn test_gp5_pick_up_down() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/pick-up-down.gp5")))
        .unwrap();
}

