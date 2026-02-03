// GP7 keysig tests
use super::super::{read_gp7};


#[test]
fn test_gp7_keysig() {
    let song = read_gp7("test/keysig.gp");
    assert_eq!(song.tracks.len(), 1);
    assert_eq!(song.measure_headers.len(), 32);
}

