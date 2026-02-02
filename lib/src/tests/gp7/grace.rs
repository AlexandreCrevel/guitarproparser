// GP7 grace tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gp7, read_file};


#[test]
fn test_gp7_grace() {
    let song = read_gp7("test/grace.gp");
    assert!(!song.tracks.is_empty());
    let graces: Vec<_> = song
        .tracks
        .iter()
        .flat_map(|t| t.measures.iter())
        .flat_map(|m| m.voices.iter())
        .flat_map(|v| v.beats.iter())
        .flat_map(|b| b.notes.iter())
        .filter_map(|n| n.effect.grace.as_ref())
        .collect();
    assert!(!graces.is_empty(), "grace.gp should contain at least one grace note");
    // Grace duration should be derived from rhythm NoteValue.
    // In-memory values: 16 = sixteenth, 32 = thirty-second, 64 = sixty-fourth.
    assert!(
        graces.iter().all(|g| g.duration == 16 || g.duration == 32 || g.duration == 64),
        "grace note duration should be a valid note value (16, 32, or 64), got: {:?}",
        graces.iter().map(|g| g.duration).collect::<Vec<_>>()
    );
}

#[test]
fn test_gp7_grace_before_beat() {
    let song = read_gp7("test/grace-before-beat.gp");
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
        "grace-before-beat.gp should contain a grace note before the beat"
    );
}

#[test]
fn test_gp7_grace_on_beat() {
    let song = read_gp7("test/grace-on-beat.gp");
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
        "grace-on-beat.gp should contain a grace note on the beat"
    );
}

