// GP3 ghost_note tests
use crate::model::song::Song;
use super::super::read_file;

#[test]
fn test_gp3_ghost_note() {
    let mut song: Song = Song::default();
    song.read_gp3(&read_file(String::from("test/ghost_note.gp3")))
        .unwrap();
}

