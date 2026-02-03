// GP7 repeats tests
use super::super::{read_gp7};


#[test]
fn test_gp7_repeats() {
    let song = read_gp7("test/repeats.gp");
    assert!(!song.measure_headers.is_empty());
    let has_repeat = song
        .measure_headers
        .iter()
        .any(|mh| mh.repeat_open || mh.repeat_close > 0);
    assert!(
        has_repeat,
        "repeats.gp should have at least one repeat marker"
    );
}

