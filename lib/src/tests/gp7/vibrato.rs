// GP7 vibrato tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gp7, read_file};


#[test]
fn test_gp7_vibrato() {
    let song = read_gp7("test/vibrato.gp");
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
        "vibrato.gp should contain at least one note with vibrato"
    );
}

