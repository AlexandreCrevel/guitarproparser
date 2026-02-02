// GP7 fret_diagram_2instruments tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gp7, read_file};


#[test]
fn test_gp7_fret_diagram_2instruments() {
    let song = read_gp7("test/fret-diagram_2instruments.gp");
    assert!(song.tracks.len() >= 2);
}

