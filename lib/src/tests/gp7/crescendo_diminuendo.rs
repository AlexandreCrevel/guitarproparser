// GP7 crescendo_diminuendo tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gp7, read_file};


#[test]
fn test_gp7_crescendo_diminuendo() {
    let song = read_gp7("test/crescendo-diminuendo.gp");
    assert!(!song.tracks.is_empty());
}

