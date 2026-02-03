// GPX dead_note tests
use super::super::{read_gpx};


#[test]
fn test_gpx_dead_note() {
    use crate::model::enums::NoteType;
    let song = read_gpx("test/dead-note.gpx");
    assert!(!song.tracks.is_empty());
    let has_dead = song.tracks.iter().any(|t| {
        t.measures.iter().any(|m| {
            m.voices.iter().any(|v| {
                v.beats
                    .iter()
                    .any(|b| b.notes.iter().any(|n| n.kind == NoteType::Dead))
            })
        })
    });
    assert!(
        has_dead,
        "dead-note.gpx should contain at least one dead note"
    );
}

