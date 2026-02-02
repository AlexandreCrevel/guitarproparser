// GP7 let_ring tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gp7, read_file};


#[test]
fn test_gp7_let_ring() {
    let song = read_gp7("test/let-ring.gp");
    assert!(!song.tracks.is_empty());
    let has_let_ring = song.tracks.iter().any(|t| {
        t.measures.iter().any(|m| {
            m.voices.iter().any(|v| {
                v.beats
                    .iter()
                    .any(|b| b.notes.iter().any(|n| n.effect.let_ring))
            })
        })
    });
    assert!(
        has_let_ring,
        "let-ring.gp should contain at least one let-ring note"
    );
}

