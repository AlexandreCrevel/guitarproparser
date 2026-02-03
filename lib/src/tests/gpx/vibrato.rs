// GPX vibrato tests
use super::super::{read_gpx};


#[test]
fn test_gpx_vibrato() {
    let song = read_gpx("test/vibrato.gpx");
    assert!(!song.tracks.is_empty());
    let has_vibrato = song.tracks.iter().any(|t| {
        t.measures.iter().any(|m| {
            m.voices.iter().any(|v| {
                v.beats
                    .iter()
                    .any(|b| b.notes.iter().any(|n| n.effect.vibrato))
            })
        })
    });
    assert!(
        has_vibrato,
        "vibrato.gpx should contain at least one note with vibrato"
    );
}

