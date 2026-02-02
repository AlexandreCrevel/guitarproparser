// GP7 slurs tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gp7, read_file};


#[test]
fn test_gp7_slur() {
    let song = read_gp7("test/slur.gp");
    assert!(!song.tracks.is_empty());
}

#[test]
fn test_gp7_slur_hammer_slur() {
    let song = read_gp7("test/slur_hammer_slur.gp");
    assert!(!song.tracks.is_empty());
}

#[test]
fn test_gp7_slur_slur_hammer() {
    let song = read_gp7("test/slur_slur_hammer.gp");
    assert!(!song.tracks.is_empty());
}

#[test]
fn test_gp7_slur_over_3_measures() {
    let song = read_gp7("test/slur_over_3_measures.gp");
    assert!(!song.tracks.is_empty());
}

#[test]
fn test_gp7_slur_voices() {
    let song = read_gp7("test/slur_voices.gp");
    assert!(!song.tracks.is_empty());
}

#[test]
fn test_gp7_slur_notes_effect_mask() {
    let song = read_gp7("test/slur-notes-effect-mask.gp");
    assert!(!song.tracks.is_empty());
}

#[test]
fn test_gp7_tuplet_with_slur() {
    let song = read_gp7("test/tuplet-with-slur.gp");
    assert!(!song.tracks.is_empty());
}

