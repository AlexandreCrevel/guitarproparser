// GP7 barre tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gp7, read_file};


#[test]
fn test_gp7_barre() {
    let song = read_gp7("test/barre.gp");
    assert!(!song.tracks.is_empty());
}

