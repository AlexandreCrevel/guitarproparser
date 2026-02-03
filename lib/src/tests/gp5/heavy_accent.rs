// GP5 heavy_accent tests
use crate::model::song::Song;
use super::super::read_file;

#[test]
fn test_gp5_heavy_accent() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/heavy-accent.gp5")))
        .unwrap();
}

