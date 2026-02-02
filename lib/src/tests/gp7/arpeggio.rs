// GP7 arpeggio tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gp7, read_file};


#[test]
fn test_gp7_arpeggio() {
    let song = read_gp7("test/arpeggio.gp");
    assert!(!song.tracks.is_empty());
    let has_arpeggio = song.tracks.iter().any(|t| {
        t.measures.iter().any(|m| {
            m.voices.iter().any(|v| {
                v.beats
                    .iter()
                    .any(|b| b.effect.stroke.direction != BeatStrokeDirection::None)
            })
        })
    });
    assert!(has_arpeggio, "arpeggio.gp should contain at least one arpeggio stroke");
}

