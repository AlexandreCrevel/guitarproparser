// GP7 ghost_note tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gp7, read_file};


#[test]
fn test_gp7_ghost_note() {
    let song = read_gp7("test/ghost-note.gp");
    assert!(!song.tracks.is_empty());
    let has_ghost = song.tracks.iter().any(|t| {
        t.measures.iter().any(|m| {
            m.voices.iter().any(|v| {
                v.beats
                    .iter()
                    .any(|b| b.notes.iter().any(|n| n.effect.ghost_note))
            })
        })
    });
    assert!(
        has_ghost,
        "ghost-note.gp should contain at least one ghost note"
    );
}

