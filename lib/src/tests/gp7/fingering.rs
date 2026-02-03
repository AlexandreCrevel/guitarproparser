// GP7 fingering tests
use super::super::{read_gp7};
use crate::types::enums::Fingering;

#[test]
fn test_gp7_fingering() {
    let song = read_gp7("test/fingering.gp");
    assert!(!song.tracks.is_empty());
    let has_left_finger = song.tracks.iter().any(|t| {
        t.measures.iter().any(|m| {
            m.voices.iter().any(|v| {
                v.beats.iter().any(|b| {
                    b.notes
                        .iter()
                        .any(|n| n.effect.left_hand_finger != Fingering::Open)
                })
            })
        })
    });
    let has_right_finger = song.tracks.iter().any(|t| {
        t.measures.iter().any(|m| {
            m.voices.iter().any(|v| {
                v.beats.iter().any(|b| {
                    b.notes
                        .iter()
                        .any(|n| n.effect.right_hand_finger != Fingering::Open)
                })
            })
        })
    });
    assert!(
        has_left_finger || has_right_finger,
        "fingering.gp should contain fingering annotations"
    );
}
