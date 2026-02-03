// GP5 unknown_chord_extension tests
use crate::model::song::Song;
use super::super::read_file;

#[test]
fn test_gp5_unknown_chord_extension() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/Unknown Chord Extension.gp5")))
        .unwrap();
}

