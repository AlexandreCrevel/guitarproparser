// GP7 tempo tests
use super::super::{read_gp7};


#[test]
fn test_gp7_tempo() {
    let song = read_gp7("test/tempo.gp");
    assert!(!song.measure_headers.is_empty());
    assert!(song.tempo > 0, "tempo should be parsed from automations");
}

