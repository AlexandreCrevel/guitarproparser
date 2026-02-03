// GP4 let_ring tests
use crate::model::song::Song;
use super::super::read_file;

#[test]
fn test_gp4_let_ring() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/let-ring.gp4")))
        .unwrap();
}

