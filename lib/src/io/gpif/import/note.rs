// GPIF note conversion
use crate::io::gpif::Note;
use crate::model::effects::*;
use crate::model::enums::*;
use crate::model::key_signature::Duration;
use crate::model::note::Note as SongNote;
use super::helpers::*;
use super::bend::{build_bend_effect, build_bend_effect_full, build_whammy_effect};

pub(crate) fn convert_note(
    g_note: &Note,
    velocity: i16,
    is_grace_beat: bool,
    grace_on_beat: bool,
    grace_duration: u8,
) -> SongNote {
    let mut s_note = SongNote {
        velocity,
        kind: NoteType::Normal,
        ..Default::default()
    };

    let mut bend_origin: Option<f64> = None;
    let mut bend_dest: Option<f64> = None;
    let mut bend_middle: Option<f64> = None;
    let mut bend_origin_offset: Option<f64> = None;
    let mut bend_middle_offset1: Option<f64> = None;
    let mut bend_dest_offset: Option<f64> = None;

    for prop in &g_note.properties.properties {
        match prop.name.as_str() {
            "Fret" => {
                if let Some(f) = prop.fret {
                    s_note.value = f as i16;
                }
            }
            "String" => {
                if let Some(s) = prop.string {
                    s_note.string = s as i8;
                }
            }
            "PalmMuted" => {
                if prop.enable.is_some() {
                    s_note.effect.palm_mute = true;
                }
            }
            "BendOriginValue" => {
                bend_origin = prop.float;
            }
            "BendDestinationValue" => {
                bend_dest = prop.float;
            }
            "BendMiddleValue" => {
                bend_middle = prop.float;
            }
            "BendOriginOffset" => {
                bend_origin_offset = prop.float;
            }
            "BendMiddleOffset1" => {
                bend_middle_offset1 = prop.float;
            }
            "BendMiddleOffset2" => {
                // GPIF provides middle_offset2, but it's not used in GP5's 3-point bend model
            }
            "BendDestinationOffset" => {
                bend_dest_offset = prop.float;
            }
            "Slide" => {
                if let Some(flags) = prop.flags {
                    s_note.effect.slides = parse_slide_flags(flags);
                }
            }
            "HarmonicType" => {
                if let Some(htype) = &prop.htype {
                    s_note.effect.harmonic = Some(parse_harmonic_type(htype));
                }
            }
            "HarmonicFret" => {
                if let Some(hfret) = prop.hfret {
                    if let Some(ref mut h) = s_note.effect.harmonic {
                        h.fret = Some(hfret as i8);
                    }
                }
            }
            "HopoOrigin" | "HopoDestination" => {
                if prop.enable.is_some() {
                    s_note.effect.hammer = true;
                }
            }
            "Dead" | "Muted" => {
                if prop.enable.is_some() {
                    s_note.kind = NoteType::Dead;
                }
            }
            // Note: "Tapped" (tap technique) is a beat-level effect (SlapEffect::Tapping),
            // handled in convert_beat after note processing.
            "Tapped" => {}
            _ => {}
        }
    }

    // Bend (with optional middle value and offsets for improved accuracy)
    if let (Some(orig), Some(dest)) = (bend_origin, bend_dest) {
        if orig != 0.0 || dest != 0.0 {
            s_note.effect.bend = Some(build_bend_effect_full(
                orig,
                bend_middle,
                dest,
                bend_origin_offset,
                bend_middle_offset1,
                bend_dest_offset,
            ));
        }
    }

    // Tie
    if let Some(tie) = &g_note.tie {
        if tie.destination == "true" {
            s_note.kind = NoteType::Tie;
        }
    }

    // Vibrato
    if g_note.vibrato.is_some() {
        s_note.effect.vibrato = true;
    }

    // Let Ring
    if g_note.let_ring.is_some() {
        s_note.effect.let_ring = true;
    }

    // Ghost note
    if g_note.anti_accent.is_some() {
        s_note.effect.ghost_note = true;
    }

    // Accent bitmask
    if let Some(accent) = g_note.accent {
        if (accent & 0x01) != 0 {
            s_note.effect.staccato = true;
        }
        if (accent & 0x02) != 0 || (accent & 0x08) != 0 {
            s_note.effect.accentuated_note = true;
        }
        if (accent & 0x04) != 0 {
            s_note.effect.heavy_accentuated_note = true;
        }
    }

    // Ornament
    if let Some(orn) = &g_note.ornament {
        s_note.effect.ornament = Some(orn.clone());
    }

    // Trill
    if let Some(trill_fret) = g_note.trill {
        s_note.effect.trill = Some(TrillEffect {
            fret: trill_fret as i8,
            duration: Duration::default(),
        });
    }

    // Fingering
    if let Some(lf) = &g_note.left_fingering {
        s_note.effect.left_hand_finger = parse_fingering(lf);
    }
    if let Some(rf) = &g_note.right_fingering {
        s_note.effect.right_hand_finger = parse_fingering(rf);
    }

    // Grace note
    if is_grace_beat {
        s_note.effect.grace = Some(GraceEffect {
            fret: s_note.value as i8,
            velocity: s_note.velocity,
            duration: grace_duration,
            is_dead: s_note.kind == NoteType::Dead,
            is_on_beat: grace_on_beat,
            transition: if s_note.effect.hammer {
                GraceEffectTransition::Hammer
            } else if !s_note.effect.slides.is_empty() {
                GraceEffectTransition::Slide
            } else {
                GraceEffectTransition::None
            },
        });
    }

    s_note
}
