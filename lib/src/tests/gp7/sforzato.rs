// GP7 sforzato tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gp7, read_file};


#[test]
fn test_gp7_sforzato() {
    let song = read_gp7("test/sforzato.gp");
    assert!(!song.tracks.is_empty());
}

