// GP7 dotted_tuplets tests
use super::super::{read_gp7};


#[test]
fn test_gp7_dotted_tuplets() {
    let song = read_gp7("test/dotted-tuplets.gp");
    assert!(!song.tracks.is_empty());
}

