// GP7 tap_slap_pop tests
use super::super::{read_gp7};
use crate::types::enums::SlapEffect;

#[test]
fn test_gp7_tap_slap_pop() {
    let song = read_gp7("test/tap-slap-pop.gp");
    assert!(!song.tracks.is_empty());
    let slap_effects: Vec<_> = song
        .tracks
        .iter()
        .flat_map(|t| t.measures.iter())
        .flat_map(|m| m.voices.iter())
        .flat_map(|v| v.beats.iter())
        .map(|b| &b.effect.slap_effect)
        .filter(|e| **e != SlapEffect::None)
        .collect();
    assert!(
        !slap_effects.is_empty(),
        "tap-slap-pop.gp should contain slap/pop/tap effects"
    );
}
