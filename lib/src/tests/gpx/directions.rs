// GPX directions tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gpx, read_file};


#[test]
fn test_gpx_directions() {
    let song = read_gpx("test/directions.gpx");
    assert!(!song.measure_headers.is_empty());
}

