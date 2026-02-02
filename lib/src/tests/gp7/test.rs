// GP7 test tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gp7, read_file};


#[test]
fn test_gp7_test() {
    let song = read_gp7("test/test.gp");
    assert!(!song.tracks.is_empty());
}

