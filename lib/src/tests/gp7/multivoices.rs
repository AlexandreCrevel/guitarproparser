// GP7 multivoices tests
use super::super::{read_gp7};


#[test]
fn test_gp7_multivoices() {
    let song = read_gp7("test/multivoices.gp");
    assert!(!song.tracks.is_empty());
}

