// GP3 slides tests
use crate::model::song::Song;
use crate::*;
use super::super::read_file;

#[test]
fn test_gp3_dotted_gliss() {
    let mut song: Song = Song::default();
    song.read_gp3(&read_file(String::from("test/dotted-gliss.gp3")))
        .unwrap();
}

