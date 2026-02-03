// GPX palm_mute tests
use super::super::{read_gpx};


#[test]
fn test_gpx_palm_mute() {
    let song = read_gpx("test/palm-mute.gpx");
    assert!(!song.tracks.is_empty());
    let has_palm_mute = song.tracks.iter().any(|t| {
        t.measures.iter().any(|m| {
            m.voices.iter().any(|v| {
                v.beats
                    .iter()
                    .any(|b| b.notes.iter().any(|n| n.effect.palm_mute))
            })
        })
    });
    assert!(
        has_palm_mute,
        "palm-mute.gpx should contain at least one palm-muted note"
    );
}

