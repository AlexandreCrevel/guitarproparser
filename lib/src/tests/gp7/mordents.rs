// GP7 mordents tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gp7, read_file};


#[test]
fn test_gp7_mordents() {
    let song = read_gp7("test/mordents.gp");
    assert!(!song.tracks.is_empty());
}

