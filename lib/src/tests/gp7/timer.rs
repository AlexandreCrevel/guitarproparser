// GP7 timer tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gp7, read_file};


#[test]
fn test_gp7_timer() {
    let song = read_gp7("test/timer.gp");
    assert!(!song.tracks.is_empty());
}

