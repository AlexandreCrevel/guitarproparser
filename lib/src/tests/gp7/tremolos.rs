// GP7 tremolos tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gp7, read_file};


#[test]
fn test_gp7_tremolos() {
    let song = read_gp7("test/tremolos.gp");
    assert!(!song.tracks.is_empty());
}

