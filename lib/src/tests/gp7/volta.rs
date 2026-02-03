// GP7 volta tests
use super::super::{read_gp7};


#[test]
fn test_gp7_volta() {
    let song = read_gp7("test/volta.gp");
    assert!(!song.measure_headers.is_empty());
    let has_volta = song
        .measure_headers
        .iter()
        .any(|mh| mh.repeat_alternative > 0);
    assert!(
        has_volta,
        "volta.gp should have at least one alternate ending"
    );
}

