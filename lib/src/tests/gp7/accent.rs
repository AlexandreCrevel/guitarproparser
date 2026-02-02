// GP7 accent tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gp7, read_file};


#[test]
fn test_gp7_accent() {
    let song = read_gp7("test/accent.gp");
    assert!(!song.tracks.is_empty());
}

