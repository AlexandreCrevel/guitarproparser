// GPIF beat conversion
use std::collections::HashMap;
use crate::io::gpif::{Beat, Note, Rhythm, Property};
use crate::model::beat::{Beat as SongBeat, Voice as SongVoice};
use crate::model::enums::*;
use crate::model::key_signature::{Duration, DURATION_EIGHTH};
use crate::model::mix_table::{MixTableChange, WahEffect};
use super::helpers::*;
use super::note::convert_note;
use super::bend::{build_bend_effect, build_whammy_effect};

pub(crate) fn convert_beat(
    g_beat: &Beat,
    rhythms_map: &HashMap<i32, &Rhythm>,
    notes_map: &HashMap<i32, &Note>,
    current_velocity: &mut i16,
) -> SongBeat {
    let mut s_beat = SongBeat::default();

    // Duration from Rhythm
    let mut grace_duration: u8 = 1;
    if let Some(rhythm_ref) = &g_beat.rhythm {
        if let Some(rhythm) = rhythms_map.get(&rhythm_ref.r#ref) {
            s_beat.duration.value = note_value_to_duration(&rhythm.note_value);
            if let Some(dot) = &rhythm.augmentation_dot {
                match dot.count {
                    1 => s_beat.duration.dotted = true,
                    2 => s_beat.duration.double_dotted = true,
                    _ => {}
                }
            }
            if let Some(tuplet) = &rhythm.primary_tuplet {
                s_beat.duration.tuplet_enters = tuplet.num as u8;
                s_beat.duration.tuplet_times = tuplet.den as u8;
            }
            // If this is a grace beat, derive grace duration from the rhythm's note value
            if g_beat.grace_notes.is_some() {
                grace_duration = grace_note_value_to_duration(&rhythm.note_value);
            }
        }
    }

    // Dynamic
    if let Some(dyn_str) = &g_beat.dynamic {
        *current_velocity = dynamic_to_velocity(dyn_str);
    }

    // Grace notes
    let is_grace_beat = g_beat.grace_notes.is_some();
    let grace_on_beat = g_beat.grace_notes.as_deref() == Some("OnBeat");

    // Text
    if let Some(text) = &g_beat.free_text {
        s_beat.text = text.clone();
    }

    // Fade in
    if let Some(fadding) = &g_beat.fadding {
        if fadding == "FadeIn" {
            s_beat.effect.fade_in = true;
        }
    }

    // Wah effect
    if let Some(wah_str) = &g_beat.wah {
        if wah_str == "Open" {
            s_beat.effect.mix_table_change = Some(MixTableChange {
                wah: Some(WahEffect {
                    value: 100, // Fully open
                    display: true,
                }),
                ..Default::default()
            });
        }
    }

    // Tremolo bar (simple)
    if let Some(tremolo_str) = &g_beat.tremolo {
        if let Ok(val) = tremolo_str.parse::<f64>() {
            if val != 0.0 {
                s_beat.effect.tremolo_bar = Some(build_bend_effect(0.0, val));
            }
        }
    }

    // Whammy bar (detailed, with middle value and offsets)
    if let Some(whammy) = &g_beat.whammy {
        s_beat.effect.tremolo_bar = Some(build_whammy_effect(whammy));
    }

    // Arpeggio
    if let Some(arp) = &g_beat.arpeggio {
        s_beat.effect.stroke.direction = match arp.as_str() {
            "Down" => BeatStrokeDirection::Down,
            "Up" => BeatStrokeDirection::Up,
            _ => BeatStrokeDirection::None,
        };
        if s_beat.effect.stroke.direction != BeatStrokeDirection::None {
            s_beat.effect.stroke.value = DURATION_EIGHTH as u16;
        }
    }

    // Ottavia (octave effects)
    if let Some(ott) = &g_beat.ottavia {
        s_beat.octave = match ott.as_str() {
            "8va" => Octave::Ottava,
            "8vb" => Octave::OttavaBassa,
            "15ma" => Octave::Quindicesima,
            "15mb" => Octave::QuindicesimaBassa,
            _ => Octave::None,
        };
    }

    // Beat properties
    if let Some(beat_props) = &g_beat.properties {
        for bp in &beat_props.properties {
            match bp.name.as_str() {
                "Brush" => {
                    if let Some(dir) = &bp.direction {
                        s_beat.effect.stroke.direction = match dir.as_str() {
                            "Down" => BeatStrokeDirection::Down,
                            "Up" => BeatStrokeDirection::Up,
                            _ => BeatStrokeDirection::None,
                        };
                        s_beat.effect.stroke.value = DURATION_EIGHTH as u16;
                    }
                }
                "Rasgueado" => {
                    s_beat.effect.has_rasgueado = true;
                }
                "PickStroke" => {
                    if let Some(dir) = &bp.direction {
                        s_beat.effect.pick_stroke = match dir.as_str() {
                            "Down" => BeatStrokeDirection::Down,
                            "Up" => BeatStrokeDirection::Up,
                            _ => BeatStrokeDirection::None,
                        };
                    }
                }
                "Slapped" => {
                    if bp.enable.is_some() {
                        s_beat.effect.slap_effect = SlapEffect::Slapping;
                    }
                }
                "Popped" => {
                    if bp.enable.is_some() {
                        s_beat.effect.slap_effect = SlapEffect::Popping;
                    }
                }
                "VibratoWTremBar" => {
                    s_beat.effect.vibrato = true;
                }
                "WhammyBar" => {
                    if s_beat.effect.tremolo_bar.is_none() {
                        if let Some(val) = bp.float {
                            if val != 0.0 {
                                s_beat.effect.tremolo_bar = Some(build_bend_effect(0.0, val));
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    // Notes
    match &g_beat.notes {
        Some(notes_str) => {
            let note_ids = parse_ids(notes_str);
            s_beat.status = if note_ids.is_empty() {
                BeatStatus::Rest
            } else {
                BeatStatus::Normal
            };

            for &nid in &note_ids {
                if let Some(g_note) = notes_map.get(&nid) {
                    let s_note = convert_note(
                        g_note,
                        *current_velocity,
                        is_grace_beat,
                        grace_on_beat,
                        grace_duration,
                    );
                    s_beat.notes.push(s_note);

                    // Tap technique is a beat-level effect (SlapEffect::Tapping)
                    if s_beat.effect.slap_effect == SlapEffect::None {
                        for prop in &g_note.properties.properties {
                            if prop.name == "Tapped" && prop.enable.is_some() {
                                s_beat.effect.slap_effect = SlapEffect::Tapping;
                                break;
                            }
                        }
                    }
                }
            }
        }
        None => {
            s_beat.status = BeatStatus::Rest;
        }
    }

    s_beat
}

