// GP7 tremolo_bar tests
use super::super::{read_gp7};


#[test]
fn test_gp7_tremolo_bar() {
    let song = read_gp7("test/tremolo-bar.gp");
    assert!(!song.tracks.is_empty());
    let has_tremolo_bar = song.tracks.iter().any(|t| {
        t.measures.iter().any(|m| {
            m.voices.iter().any(|v| {
                v.beats.iter().any(|b| b.effect.tremolo_bar.is_some())
            })
        })
    });
    assert!(
        has_tremolo_bar,
        "tremolo-bar.gp should contain at least one whammy bar effect"
    );
}

