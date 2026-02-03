// GP5 tempo tests
use crate::model::song::Song;
use super::super::read_file;

#[test]
fn test_gp5_tempo() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/tempo.gp5")))
        .unwrap();
}

