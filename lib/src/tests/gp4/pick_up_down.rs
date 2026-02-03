// GP4 pick_up_down tests
use crate::model::song::Song;
use super::super::read_file;

#[test]
fn test_gp4_pick_up_down() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/pick-up-down.gp4")))
        .unwrap();
}

