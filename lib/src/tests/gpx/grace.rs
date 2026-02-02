// GPX grace tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gpx, read_file};


#[test]
fn test_gpx_grace() {
    let song = read_gpx("test/grace.gpx");
    assert!(!song.tracks.is_empty());
    let has_grace = song.tracks.iter().any(|t| {
        t.measures.iter().any(|m| {
            m.voices.iter().any(|v| {
                v.beats
                    .iter()
                    .any(|b| b.notes.iter().any(|n| n.effect.grace.is_some()))
            })
        })
    });
    assert!(
        has_grace,
        "grace.gpx should contain at least one grace note"
    );
}

#[test]
fn test_gpx_grace_before_beat() {
    let song = read_gpx("test/grace-before-beat.gpx");
    assert!(!song.tracks.is_empty());
    let has_grace_before = song.tracks.iter().any(|t| {
        t.measures.iter().any(|m| {
            m.voices.iter().any(|v| {
                v.beats.iter().any(|b| {
                    b.notes
                        .iter()
                        .any(|n| n.effect.grace.as_ref().is_some_and(|g| !g.is_on_beat))
                })
            })
        })
    });
    assert!(
        has_grace_before,
        "grace-before-beat.gpx should contain a grace note before the beat"
    );
}

#[test]
fn test_gpx_grace_on_beat() {
    let song = read_gpx("test/grace-on-beat.gpx");
    assert!(!song.tracks.is_empty());
    let has_grace_on = song.tracks.iter().any(|t| {
        t.measures.iter().any(|m| {
            m.voices.iter().any(|v| {
                v.beats.iter().any(|b| {
                    b.notes
                        .iter()
                        .any(|n| n.effect.grace.as_ref().is_some_and(|g| g.is_on_beat))
                })
            })
        })
    });
    assert!(
        has_grace_on,
        "grace-on-beat.gpx should contain a grace note on the beat"
    );
}

