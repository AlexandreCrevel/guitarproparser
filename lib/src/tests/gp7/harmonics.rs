// GP7 harmonics tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gp7, read_file};


#[test]
fn test_gp7_artificial_harmonic() {
    let song = read_gp7("test/artificial-harmonic.gp");
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
        "artificial-harmonic.gp should contain at least one harmonic note"
    );
}

