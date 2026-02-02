// GP7 trill tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gp7, read_file};


#[test]
fn test_gp7_trill() {
    let song = read_gp7("test/trill.gp");
    assert!(!song.tracks.is_empty());
    let has_trill = song.tracks.iter().any(|t| {
        t.measures.iter().any(|m| {
            m.voices.iter().any(|v| {
                v.beats
                    .iter()
                    .any(|b| b.notes.iter().any(|n| n.effect.trill.is_some()))
            })
        })
    });
    assert!(has_trill, "trill.gp should contain at least one trill note");
}

