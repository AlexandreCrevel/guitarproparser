// GP7 clefs tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gp7, read_file};


#[test]
fn test_gp7_clefs() {
    let song = read_gp7("test/clefs.gp");
    assert!(!song.tracks.is_empty());
}

