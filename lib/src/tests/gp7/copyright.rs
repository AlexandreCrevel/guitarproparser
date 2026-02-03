// GP7 copyright tests
use super::super::{read_gp7};


#[test]
fn test_gp7_copyright() {
    let song = read_gp7("test/copyright.gp");
    assert!(!song.tracks.is_empty());
    assert!(
        !song.copyright.is_empty(),
        "copyright field should be populated"
    );
}

