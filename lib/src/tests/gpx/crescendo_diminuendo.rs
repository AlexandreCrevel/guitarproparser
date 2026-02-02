// GPX crescendo_diminuendo tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gpx, read_file};


#[test]
fn test_gpx_crescendo_diminuendo() {
    let song = read_gpx("test/crescendo-diminuendo.gpx");
    assert!(!song.tracks.is_empty());
}

