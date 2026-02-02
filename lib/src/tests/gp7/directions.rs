// GP7 directions tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gp7, read_file};


#[test]
fn test_gp7_directions() {
    let song = read_gp7("test/directions.gp");
    assert!(!song.measure_headers.is_empty());
}

