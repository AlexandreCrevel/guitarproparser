// GP7 high_pitch tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gp7, read_file};


#[test]
fn test_gp7_high_pitch() {
    let song = read_gp7("test/high-pitch.gp");
    assert!(!song.tracks.is_empty());
}

