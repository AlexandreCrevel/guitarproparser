// GP4 palm_mute tests
use crate::model::song::Song;
use super::super::read_file;

#[test]
fn test_gp4_palm_mute() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/palm-mute.gp4")))
        .unwrap();
}

