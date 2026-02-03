// GPX let_ring tests
use super::super::{read_gpx};


#[test]
fn test_gpx_let_ring() {
    let song = read_gpx("test/let-ring.gpx");
    assert!(!song.tracks.is_empty());
    let has_let_ring = song.tracks.iter().any(|t| {
        t.measures.iter().any(|m| {
            m.voices.iter().any(|v| {
                v.beats
                    .iter()
                    .any(|b| b.notes.iter().any(|n| n.effect.let_ring))
            })
        })
    });
    assert!(
        has_let_ring,
        "let-ring.gpx should contain at least one let-ring note"
    );
}

