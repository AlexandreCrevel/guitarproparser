// GP7 palm_mute tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gp7, read_file};


#[test]
fn test_gp7_palm_mute() {
    let song = read_gp7("test/palm-mute.gp");
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
        "palm-mute.gp should contain at least one palm-muted note"
    );
}

