// GPX slurs tests
use super::super::{read_gpx};


#[test]
fn test_gpx_slur() {
    let song = read_gpx("test/slur.gpx");
    assert!(!song.tracks.is_empty());
}

#[test]
fn test_gpx_slur_hammer_slur() {
    let song = read_gpx("test/slur_hammer_slur.gpx");
    assert!(!song.tracks.is_empty());
}

#[test]
fn test_gpx_slur_slur_hammer() {
    let song = read_gpx("test/slur_slur_hammer.gpx");
    assert!(!song.tracks.is_empty());
}

#[test]
fn test_gpx_slur_over_3_measures() {
    let song = read_gpx("test/slur_over_3_measures.gpx");
    assert!(!song.tracks.is_empty());
}

#[test]
fn test_gpx_slur_voices() {
    let song = read_gpx("test/slur_voices.gpx");
    assert!(!song.tracks.is_empty());
}

#[test]
fn test_gpx_slur_notes_effect_mask() {
    let song = read_gpx("test/slur-notes-effect-mask.gpx");
    assert!(!song.tracks.is_empty());
}

#[test]
fn test_gpx_tuplet_with_slur() {
    let song = read_gpx("test/tuplet-with-slur.gpx");
    assert!(!song.tracks.is_empty());
}

