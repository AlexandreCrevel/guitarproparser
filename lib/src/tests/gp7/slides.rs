// GP7 slides tests
use super::super::{read_gp7};


#[test]
fn test_gp7_shift_slide() {
    let song = read_gp7("test/shift-slide.gp");
    assert!(!song.tracks.is_empty());
    let has_slide = song.tracks.iter().any(|t| {
        t.measures.iter().any(|m| {
            m.voices.iter().any(|v| {
                v.beats
                    .iter()
                    .any(|b| b.notes.iter().any(|n| !n.effect.slides.is_empty()))
            })
        })
    });
    assert!(
        has_slide,
        "shift-slide.gp should contain at least one note with slide effect"
    );
}

#[test]
fn test_gp7_legato_slide() {
    let song = read_gp7("test/legato-slide.gp");
    assert!(!song.tracks.is_empty());
}

#[test]
fn test_gp7_slide_out_down() {
    let song = read_gp7("test/slide-out-down.gp");
    assert!(!song.tracks.is_empty());
}

#[test]
fn test_gp7_slide_out_up() {
    let song = read_gp7("test/slide-out-up.gp");
    assert!(!song.tracks.is_empty());
}

#[test]
fn test_gp7_slide_in_below() {
    let song = read_gp7("test/slide-in-below.gp");
    assert!(!song.tracks.is_empty());
}

#[test]
fn test_gp7_slide_in_above() {
    let song = read_gp7("test/slide-in-above.gp");
    assert!(!song.tracks.is_empty());
}

#[test]
fn test_gp7_dotted_gliss() {
    let song = read_gp7("test/dotted-gliss.gp");
    assert!(!song.tracks.is_empty());
}

