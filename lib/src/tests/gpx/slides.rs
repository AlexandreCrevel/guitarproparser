// GPX slides tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gpx, read_file};


#[test]
fn test_gpx_shift_slide() {
    let song = read_gpx("test/shift-slide.gpx");
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
        "shift-slide.gpx should contain at least one note with slide effect"
    );
}

#[test]
fn test_gpx_legato_slide() {
    let song = read_gpx("test/legato-slide.gpx");
    assert!(!song.tracks.is_empty());
}

#[test]
fn test_gpx_slide_out_down() {
    let song = read_gpx("test/slide-out-down.gpx");
    assert!(!song.tracks.is_empty());
}

#[test]
fn test_gpx_slide_out_up() {
    let song = read_gpx("test/slide-out-up.gpx");
    assert!(!song.tracks.is_empty());
}

#[test]
fn test_gpx_slide_in_below() {
    let song = read_gpx("test/slide-in-below.gpx");
    assert!(!song.tracks.is_empty());
}

#[test]
fn test_gpx_slide_in_above() {
    let song = read_gpx("test/slide-in-above.gpx");
    assert!(!song.tracks.is_empty());
}

#[test]
fn test_gpx_dotted_gliss() {
    let song = read_gpx("test/dotted-gliss.gpx");
    assert!(!song.tracks.is_empty());
}

