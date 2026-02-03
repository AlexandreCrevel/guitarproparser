// GP7 fret_diagram_2instruments tests
use super::super::{read_gp7};


#[test]
fn test_gp7_fret_diagram_2instruments() {
    let song = read_gp7("test/fret-diagram_2instruments.gp");
    assert!(song.tracks.len() >= 2);
}

