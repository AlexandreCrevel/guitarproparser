// GP7 wah tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gp7, read_file};


#[test]
fn test_gp7_wah() {
    let song = read_gp7("test/wah.gp");
    assert!(!song.tracks.is_empty());
}

