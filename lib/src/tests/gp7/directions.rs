// GP7 directions tests
use super::super::{read_gp7};


#[test]
fn test_gp7_directions() {
    let song = read_gp7("test/directions.gp");
    assert!(!song.measure_headers.is_empty());
}

