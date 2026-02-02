// GPX dynamic tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gpx, read_file};


#[test]
fn test_gpx_dynamic() {
    let song = read_gpx("test/dynamic.gpx");
    assert!(!song.tracks.is_empty());
    // Verify that notes have varying velocities (not all the same default)
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
    assert!(!velocities.is_empty(), "dynamic.gpx should contain notes");
    let has_varying = velocities.iter().any(|&v| v != velocities[0]);
    assert!(
        has_varying,
        "dynamic.gpx should have varying velocities across notes"
    );
}

