// GP7 fade_in tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gp7, read_file};


#[test]
fn test_gp7_fade_in() {
    let song = read_gp7("test/fade-in.gp");
    assert!(!song.tracks.is_empty());
    let has_fade_in = song.tracks.iter().any(|t| {
        t.measures.iter().any(|m| {
            m.voices
                .iter()
                .any(|v| v.beats.iter().any(|b| b.effect.fade_in))
        })
    });
    assert!(
        has_fade_in,
        "fade-in.gp should contain at least one beat with fade-in"
    );
}

