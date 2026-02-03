// GPX fade_in tests
use super::super::{read_gpx};


#[test]
fn test_gpx_fade_in() {
    let song = read_gpx("test/fade-in.gpx");
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
        "fade-in.gpx should contain at least one beat with fade-in"
    );
}

