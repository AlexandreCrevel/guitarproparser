// GPX trill tests
use super::super::{read_gpx};


#[test]
fn test_gpx_trill() {
    let song = read_gpx("test/trill.gpx");
    assert!(!song.tracks.is_empty());
    let has_trill = song.tracks.iter().any(|t| {
        t.measures.iter().any(|m| {
            m.voices.iter().any(|v| {
                v.beats
                    .iter()
                    .any(|b| b.notes.iter().any(|n| n.effect.trill.is_some()))
            })
        })
    });
    assert!(
        has_trill,
        "trill.gpx should contain at least one trill note"
    );
}

