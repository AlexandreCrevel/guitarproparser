// GPX ghost_note tests
use super::super::{read_gpx};


#[test]
fn test_gpx_ghost_note() {
    let song = read_gpx("test/ghost-note.gpx");
    assert!(!song.tracks.is_empty());
    let has_ghost = song.tracks.iter().any(|t| {
        t.measures.iter().any(|m| {
            m.voices.iter().any(|v| {
                v.beats
                    .iter()
                    .any(|b| b.notes.iter().any(|n| n.effect.ghost_note))
            })
        })
    });
    assert!(
        has_ghost,
        "ghost-note.gpx should contain at least one ghost note"
    );
}

