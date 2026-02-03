// GP5 wah_m tests
use crate::model::song::Song;
use super::super::read_file;

#[test]
fn test_gp5_wah_m() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/Wah-m.gp5")))
        .unwrap();
}

