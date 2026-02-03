// GP5 let_ring tests
use crate::model::song::Song;
use super::super::read_file;

#[test]
fn test_gp5_let_ring() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/let-ring.gp5")))
        .unwrap();
}

