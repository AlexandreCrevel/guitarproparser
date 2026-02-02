// GPX bend tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gpx, read_file};


#[test]
fn test_gpx_bend() {
    let song = read_gpx("test/bend.gpx");
    assert!(!song.tracks.is_empty());
    // Verify that at least one note has a bend effect
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
        "bend.gpx should contain at least one note with a bend effect"
    );
}

