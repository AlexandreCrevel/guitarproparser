// GPX harmonics tests
use super::super::{read_gpx};


#[test]
fn test_gpx_artificial_harmonic() {
    let song = read_gpx("test/artificial-harmonic.gpx");
    assert!(!song.tracks.is_empty());
    let has_harmonic = song.tracks.iter().any(|t| {
        t.measures.iter().any(|m| {
            m.voices.iter().any(|v| {
                v.beats
                    .iter()
                    .any(|b| b.notes.iter().any(|n| n.effect.harmonic.is_some()))
            })
        })
    });
    assert!(
        has_harmonic,
        "artificial-harmonic.gpx should contain at least one harmonic note"
    );
}

