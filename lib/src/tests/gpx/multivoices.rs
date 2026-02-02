// GPX multivoices tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gpx, read_file};


#[test]
fn test_gpx_multivoices() {
    let song = read_gpx("test/multivoices.gpx");
    assert!(!song.tracks.is_empty());
}

