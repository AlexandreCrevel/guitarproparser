// GP7 bend tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gp7, read_file};


#[test]
fn test_gp7_bend() {
    let song = read_gp7("test/bend.gp");
    assert!(!song.tracks.is_empty());
    let has_bend = song.tracks.iter().any(|t| {
        t.measures.iter().any(|m| {
            m.voices.iter().any(|v| {
                v.beats
                    .iter()
                    .any(|b| b.notes.iter().any(|n| n.effect.bend.is_some()))
            })
        })
    });
    assert!(
        has_bend,
        "bend.gp should contain at least one note with a bend effect"
    );
}

