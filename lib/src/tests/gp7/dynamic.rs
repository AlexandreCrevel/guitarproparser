// GP7 dynamic tests
use super::super::{read_gp7};


#[test]
fn test_gp7_dynamic() {
    let song = read_gp7("test/dynamic.gp");
    assert!(!song.tracks.is_empty());
    let velocities: Vec<i16> = song
        .tracks
        .iter()
        .flat_map(|t| {
            t.measures.iter().flat_map(|m| {
                m.voices.iter().flat_map(|v| {
                    v.beats
                        .iter()
                        .flat_map(|b| b.notes.iter().map(|n| n.velocity))
                })
            })
        })
        .collect();
    assert!(!velocities.is_empty(), "dynamic.gp should contain notes");
    let has_varying = velocities.iter().any(|&v| v != velocities[0]);
    assert!(
        has_varying,
        "dynamic.gp should have varying velocities across notes"
    );
}

