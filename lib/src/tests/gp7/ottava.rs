// GP7 ottava tests
use super::super::{read_gp7};
use crate::types::enums::Octave;

#[test]
fn test_gp7_ottava1() {
    let song = read_gp7("test/ottava1.gp");
    assert!(!song.tracks.is_empty());
    let has_ottava = song.tracks.iter().any(|t| {
        t.measures.iter().any(|m| {
            m.voices
                .iter()
                .any(|v| v.beats.iter().any(|b| b.octave == Octave::Ottava))
        })
    });
    assert!(has_ottava, "ottava1.gp should contain an 8va octave effect");
}

#[test]
fn test_gp7_ottava2() {
    let song = read_gp7("test/ottava2.gp");
    assert!(!song.tracks.is_empty());
}

#[test]
fn test_gp7_ottava3() {
    let song = read_gp7("test/ottava3.gp");
    assert!(!song.tracks.is_empty());
}

#[test]
fn test_gp7_ottava4() {
    let song = read_gp7("test/ottava4.gp");
    assert!(!song.tracks.is_empty());
}

#[test]
fn test_gp7_ottava5() {
    let song = read_gp7("test/ottava5.gp");
    assert!(!song.tracks.is_empty());
}
