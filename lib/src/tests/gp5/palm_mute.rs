// GP5 palm_mute tests
use crate::model::song::Song;
use super::super::read_file;

#[test]
fn test_gp5_palm_mute() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/palm-mute.gp5")))
        .unwrap();
}

