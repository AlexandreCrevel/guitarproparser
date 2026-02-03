// GP7 free_time tests
use super::super::{read_gp7};


#[test]
fn test_gp7_free_time() {
    let song = read_gp7("test/free-time.gp");
    assert!(!song.tracks.is_empty());
}

