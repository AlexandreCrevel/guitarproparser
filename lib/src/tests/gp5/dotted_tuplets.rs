// GP5 dotted_tuplets tests
use crate::model::song::Song;
use super::super::read_file;

#[test]
fn test_gp5_dotted_tuplets() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/dotted-tuplets.gp5")))
        .unwrap();
}

