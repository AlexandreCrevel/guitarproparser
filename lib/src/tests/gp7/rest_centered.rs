// GP7 rest_centered tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gp7, read_file};


#[test]
fn test_gp7_rest_centered() {
    let song = read_gp7("test/rest-centered.gp");
    assert!(!song.tracks.is_empty());
}

