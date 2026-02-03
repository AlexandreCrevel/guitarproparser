// GPX fret_diagram_2instruments tests
use super::super::{read_gpx};


#[test]
fn test_gpx_fret_diagram_2instruments() {
    let song = read_gpx("test/fret-diagram_2instruments.gpx");
    assert!(song.tracks.len() >= 2);
}

