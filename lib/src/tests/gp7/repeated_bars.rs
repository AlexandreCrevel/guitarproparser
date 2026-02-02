// GP7 repeated_bars tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gp7, read_file};


#[test]
fn test_gp7_repeated_bars() {
    let song = read_gp7("test/repeated-bars.gp");
    assert!(!song.tracks.is_empty());
}

