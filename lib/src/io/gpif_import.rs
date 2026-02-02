use std::collections::HashMap;

use crate::io::gpif::*;
use crate::model::{
    beat::{Beat as SongBeat, Voice as SongVoice},
    effects::*,
    enums::*,
    headers::{FermataType, Marker, MeasureFermata, MeasureHeader},
    key_signature::*,
    measure::Measure,
    mix_table::*,
    note::Note as SongNote,
    song::*,
    track::Track as SongTrack,
};

pub trait SongGpifOps {
    fn read_gpif(&mut self, gpif: &Gpif);
}

// ---------------------------------------------------------------------------
// Helper functions
// ---------------------------------------------------------------------------

/// Convert GPIF note value string to Duration.value.
/// Falls back to Quarter (4) for unknown values.
fn note_value_to_duration(s: &str) -> u16 {
    match s {
        "Whole" => 1,
        "Half" => 2,
        "Quarter" => 4,
        "Eighth" => 8,
        "16th" => 16,
        "32nd" => 32,
        "64th" => 64,
        "128th" => 128,
        _ => 4,
    }
}

/// Convert GPIF grace note rhythm NoteValue to GP5-compatible grace duration.
/// GP5 encodes grace duration as: 1 = sixteenth, 2 = twenty-fourth, 3 = thirty-second.
fn grace_note_value_to_duration(s: &str) -> u8 {
    match s {
        "16th" => 1,
        "32nd" => 2,
        "64th" => 3,
        _ => 1,
    }
}

/// Convert GPIF dynamic string to MIDI velocity
fn dynamic_to_velocity(s: &str) -> i16 {
    match s {
        "PPP" => MIN_VELOCITY,
        "PP" => MIN_VELOCITY + VELOCITY_INCREMENT,
        "P" => MIN_VELOCITY + VELOCITY_INCREMENT * 2,
        "MP" => MIN_VELOCITY + VELOCITY_INCREMENT * 3,
        "MF" => MIN_VELOCITY + VELOCITY_INCREMENT * 4,
        "F" => FORTE,
        "FF" => MIN_VELOCITY + VELOCITY_INCREMENT * 6,
        "FFF" => MIN_VELOCITY + VELOCITY_INCREMENT * 7,
        _ => FORTE,
    }
}

/// Parse space-separated integer IDs from a string.
fn parse_ids(s: &str) -> Vec<i32> {
    s.split_whitespace()
        .filter_map(|tok| tok.parse::<i32>().ok())
        .collect()
}

/// Parse slide flags bitmask into a list of `SlideType` values.
///
/// Uses the same encoding as GP5 binary format:
/// - bit 0 (0x01): Shift slide to next note
/// - bit 1 (0x02): Legato slide to next note
/// - bit 2 (0x04): Slide out downwards
/// - bit 3 (0x08): Slide out upwards
/// - bit 4 (0x10): Slide in from below
/// - bit 5 (0x20): Slide in from above
fn parse_slide_flags(flags: i32) -> Vec<SlideType> {
    let mut v = Vec::with_capacity(6);
    if (flags & 0x01) != 0 {
        v.push(SlideType::ShiftSlideTo);
    }
    if (flags & 0x02) != 0 {
        v.push(SlideType::LegatoSlideTo);
    }
    if (flags & 0x04) != 0 {
        v.push(SlideType::OutDownwards);
    }
    if (flags & 0x08) != 0 {
        v.push(SlideType::OutUpWards);
    }
    if (flags & 0x10) != 0 {
        v.push(SlideType::IntoFromBelow);
    }
    if (flags & 0x20) != 0 {
        v.push(SlideType::IntoFromAbove);
    }
    v
}

/// Parse a GPIF harmonic type string (e.g. "Natural", "Artificial", "Pinch")
/// into a `HarmonicEffect`. Falls back to `Natural` for unrecognised values.
/// "Feedback" is mapped to `Pinch` as Guitar Pro treats them equivalently.
fn parse_harmonic_type(htype: &str) -> HarmonicEffect {
    let kind = match htype {
        "Natural" => HarmonicType::Natural,
        "Artificial" => HarmonicType::Artificial,
        "Pinch" => HarmonicType::Pinch,
        "Tap" | "Tapped" => HarmonicType::Tapped,
        "Semi" => HarmonicType::Semi,
        "Feedback" => HarmonicType::Pinch,
        _ => HarmonicType::Natural,
    };
    HarmonicEffect {
        kind,
        ..Default::default()
    }
}

/// Parse direction string to DirectionSign enum.
fn parse_direction_sign(s: &str) -> Option<DirectionSign> {
    match s {
        "Coda" => Some(DirectionSign::Coda),
        "DoubleCoda" => Some(DirectionSign::DoubleCoda),
        "Segno" => Some(DirectionSign::Segno),
        "SegnoSegno" => Some(DirectionSign::SegnoSegno),
        "Fine" => Some(DirectionSign::Fine),
        "DaCapo" => Some(DirectionSign::DaCapo),
        "DaCapoAlCoda" => Some(DirectionSign::DaCapoAlCoda),
        "DaCapoAlDoubleCoda" => Some(DirectionSign::DaCapoAlDoubleCoda),
        "DaCapoAlFine" => Some(DirectionSign::DaCapoAlFine),
        "DaSegno" => Some(DirectionSign::DaSegno),
        "DaSegnoAlCoda" => Some(DirectionSign::DaSegnoAlCoda),
        "DaSegnoAlDoubleCoda" => Some(DirectionSign::DaSegnoAlDoubleCoda),
        "DaSegnoAlFine" => Some(DirectionSign::DaSegnoAlFine),
        "DaSegnoSegno" => Some(DirectionSign::DaSegnoSegno),
        "DaSegnoSegnoAlCoda" => Some(DirectionSign::DaSegnoSegnoAlCoda),
        "DaSegnoSegnoAlDoubleCoda" => Some(DirectionSign::DaSegnoSegnoAlDoubleCoda),
        "DaSegnoSegnoAlFine" => Some(DirectionSign::DaSegnoSegnoAlFine),
        "DaCoda" => Some(DirectionSign::DaCoda),
        "DaDoubleCoda" => Some(DirectionSign::DaDoubleCoda),
        _ => None,
    }
}

/// Build bend effect from GPIF origin/middle/destination values (float, in 1/100 semitone).
/// When middle value and offsets are provided, uses them for a more accurate 3-point curve.
fn build_bend_effect_full(
    origin: f64,
    middle: Option<f64>,
    destination: f64,
    origin_offset: Option<f64>,
    middle_offset1: Option<f64>,
    _middle_offset2: Option<f64>,
    destination_offset: Option<f64>,
) -> BendEffect {
    let mut bend = BendEffect::default();
    let origin_val = (origin / GP_BEND_SEMITONE as f64).round() as i8;
    let dest_val = (destination / GP_BEND_SEMITONE as f64).round() as i8;

    if origin == 0.0 && destination > 0.0 {
        bend.kind = BendType::Bend;
    } else if origin > 0.0 && destination == 0.0 {
        bend.kind = BendType::ReleaseUp;
    } else if origin > 0.0 && destination > 0.0 {
        if destination > origin {
            bend.kind = BendType::Bend;
        } else if destination < origin {
            bend.kind = BendType::ReleaseUp;
        } else {
            bend.kind = BendType::Bend;
        }
    }

    bend.value = (destination.max(origin) / GP_BEND_SEMITONE as f64 * 2.0).round() as i16;

    // Compute positions (0-12 range). GPIF offsets are percentages (0-100).
    let p0 = origin_offset
        .map(|o| (o / 100.0 * BEND_EFFECT_MAX_POSITION as f64).round() as u8)
        .unwrap_or(0);
    let p2 = destination_offset
        .map(|o| (o / 100.0 * BEND_EFFECT_MAX_POSITION as f64).round() as u8)
        .unwrap_or(BEND_EFFECT_MAX_POSITION);

    let mid_val = middle
        .map(|m| (m / GP_BEND_SEMITONE as f64).round() as i8)
        .unwrap_or(((origin_val as i16 + dest_val as i16) / 2) as i8);
    let p1 = middle_offset1
        .map(|o| (o / 100.0 * BEND_EFFECT_MAX_POSITION as f64).round() as u8)
        .unwrap_or(BEND_EFFECT_MAX_POSITION / 2);

    bend.points.push(BendPoint {
        position: p0,
        value: origin_val,
        vibrato: false,
    });
    bend.points.push(BendPoint {
        position: p1,
        value: mid_val,
        vibrato: false,
    });
    bend.points.push(BendPoint {
        position: p2,
        value: dest_val,
        vibrato: false,
    });
    bend
}

/// Build bend effect from simple origin/destination values (backwards compatible).
fn build_bend_effect(origin: f64, destination: f64) -> BendEffect {
    build_bend_effect_full(origin, None, destination, None, None, None, None)
}

/// Build a whammy bar bend effect from GPIF Whammy element attributes.
fn build_whammy_effect(w: &WhammyInfo) -> BendEffect {
    build_bend_effect_full(
        w.origin_value,
        Some(w.middle_value),
        w.destination_value,
        Some(w.origin_offset),
        Some(w.middle_offset1),
        Some(w.middle_offset2),
        Some(w.destination_offset),
    )
}

/// Extract tuning pitches from a property list.
fn extract_tuning(properties: &[Property]) -> Vec<(i8, i8)> {
    for prop in properties {
        if prop.name == "Tuning" {
            if let Some(pitches_str) = &prop.pitches {
                let pitches: Vec<i8> = pitches_str
                    .split_whitespace()
                    .filter_map(|s| s.parse::<i8>().ok())
                    .collect();
                return pitches
                    .iter()
                    .enumerate()
                    .map(|(i, &pitch)| ((i + 1) as i8, pitch))
                    .collect();
            }
        }
    }
    Vec::new()
}

/// Parse GPIF fingering string to Fingering enum.
fn parse_fingering(s: &str) -> Fingering {
    match s {
        "Open" => Fingering::Open,
        "P" => Fingering::Thumb,
        "I" => Fingering::Index,
        "M" => Fingering::Middle,
        "A" => Fingering::Annular,
        "C" => Fingering::Little,
        _ => Fingering::Open,
    }
}

/// Parse GPIF fermata type string to FermataType enum.
fn parse_fermata_type(s: &str) -> FermataType {
    match s {
        "Short" => FermataType::Short,
        "Long" => FermataType::Long,
        _ => FermataType::Medium,
    }
}

/// Parse GPIF version string (e.g. "7") into a version tuple.
/// Falls back to the provided default if parsing fails.
fn parse_gpif_version(gpif: &Gpif, default: (u8, u8, u8)) -> (u8, u8, u8) {
    if let Some(ver_str) = &gpif.version {
        let parts: Vec<u8> = ver_str
            .split('.')
            .filter_map(|s| s.parse::<u8>().ok())
            .collect();
        return match parts.len() {
            1 => (parts[0], 0, 0),
            2 => (parts[0], parts[1], 0),
            3.. => (parts[0], parts[1], parts[2]),
            _ => default,
        };
    }
    default
}

// ---------------------------------------------------------------------------
// Main conversion
// ---------------------------------------------------------------------------

impl SongGpifOps for Song {
    fn read_gpif(&mut self, gpif: &Gpif) {
        // 0. Version
        let default_version = self.version.number;
        self.version.number = parse_gpif_version(gpif, default_version);

        // 1. Metadata
        self.name = gpif.score.title.clone();
        self.subtitle = gpif.score.sub_title.clone();
        self.artist = gpif.score.artist.clone();
        self.album = gpif.score.album.clone();
        self.words = gpif.score.words.clone();
        self.author = gpif.score.music.clone();
        self.writer = gpif.score.music.clone();
        self.transcriber = gpif.score.tabber.clone();
        self.copyright = gpif.score.copyright.clone();
        self.comments = gpif.score.instructions.clone();
        // Notices
        if !gpif.score.notices.is_empty() {
            self.notice = gpif.score.notices.lines().map(|l| l.to_string()).collect();
        }

        // 2. Tempo from MasterTrack automations
        if let Some(automations) = &gpif.master_track.automations {
            for auto in &automations.automations {
                if auto.automation_type == "Tempo" && auto.bar == 0 {
                    if let Some(tempo_str) = auto.value.split_whitespace().next() {
                        self.tempo = match tempo_str.parse::<f64>() {
                            Ok(v) => v as i16,
                            Err(_) => 120,
                        };
                    }
                }
            }
        }

        // 2b. Master RSE
        if let Some(rse_wrapper) = &gpif.master_track.rse {
            if let Some(master) = &rse_wrapper.master {
                if let Some(vol) = master.volume {
                    // GPX volume is typically 0.0-1.0 or similar.
                    // Scorelib model expects arbitrary flow. we store as is.
                    self.master_effect.volume = vol * 100.0; // rudimentary mapping
                }
            }
        }

        // 3. Build lookup maps
        let bars_map: HashMap<i32, &Bar> = gpif.bars.bars.iter().map(|b| (b.id, b)).collect();
        let voices_map: HashMap<i32, &Voice> =
            gpif.voices.voices.iter().map(|v| (v.id, v)).collect();
        let beats_map: HashMap<i32, &Beat> = gpif.beats.beats.iter().map(|b| (b.id, b)).collect();
        let notes_map: HashMap<i32, &Note> = gpif.notes.notes.iter().map(|n| (n.id, n)).collect();
        let rhythms_map: HashMap<i32, &Rhythm> =
            gpif.rhythms.rhythms.iter().map(|r| (r.id, r)).collect();

        // 4. Measure Headers (MasterBars) — also collects per-track bar IDs
        self.measure_headers.clear();
        let num_tracks = gpif.tracks.tracks.len();
        let mut track_bar_ids: Vec<Vec<i32>> = vec![Vec::new(); num_tracks];

        for (mh_idx, mb) in gpif.master_bars.master_bars.iter().enumerate() {
            let mut mh = MeasureHeader {
                number: (mh_idx + 1) as u16,
                ..Default::default()
            };

            // Time signature
            let time_parts: Vec<&str> = mb.time.split('/').collect();
            if time_parts.len() == 2 {
                mh.time_signature.numerator = time_parts[0].parse().unwrap_or(4) as i8;
                mh.time_signature.denominator.value = time_parts[1].parse().unwrap_or(4) as u16;
            }

            // Key signature
            if let Some(key) = &mb.key {
                mh.key_signature.key = key.accidental_count as i8;
                mh.key_signature.is_minor = key.mode == "Minor";
            }

            // Tempo at this bar
            if let Some(automations) = &gpif.master_track.automations {
                for auto in &automations.automations {
                    if auto.automation_type == "Tempo" && auto.bar == mh_idx as i32 {
                        if let Some(tempo_str) = auto.value.split_whitespace().next() {
                            mh.tempo = tempo_str.parse::<f64>().unwrap_or(0.0) as i32;
                        }
                    }
                }
            }

            // Repeat
            if let Some(repeat) = &mb.repeat {
                mh.repeat_open = repeat.start == "true";
                if repeat.end == "true" {
                    mh.repeat_close = repeat.count.max(1) as i8;
                }
            }

            // Alternate endings (volta)
            if let Some(alt_str) = &mb.alternate_endings {
                let mut bitmask: u8 = 0;
                for tok in alt_str.split_whitespace() {
                    if let Ok(n) = tok.parse::<u8>() {
                        if n > 0 && n <= 8 {
                            bitmask |= 1 << (n - 1);
                        }
                    }
                }
                mh.repeat_alternative = bitmask;
            }

            // Double bar
            mh.double_bar = mb.double_bar.is_some();

            // Marker (Section)
            if let Some(section) = &mb.section {
                let title = section
                    .text
                    .as_deref()
                    .unwrap_or(section.letter.as_deref().unwrap_or("Section"));
                // GP6/7 GPIF XML does not include marker color; use the default (red).
                mh.marker = Some(Marker {
                    title: title.to_string(),
                    color: 0xff0000,
                });
            }

            // Fermatas
            if let Some(fermatas_w) = &mb.fermatas {
                for f in &fermatas_w.fermatas {
                    let ftype = parse_fermata_type(
                        f.fermata_type.as_deref().unwrap_or("Medium"),
                    );
                    let offset = f.offset.as_deref().unwrap_or("").to_string();
                    mh.fermatas.push(MeasureFermata {
                        fermata_type: ftype,
                        offset,
                    });
                }
            }

            // Free time
            mh.free_time = mb.free_time.is_some();

            // Directions
            if let Some(dirs) = &mb.directions {
                if let Some(target) = &dirs.target {
                    mh.direction = parse_direction_sign(target);
                } else if let Some(jump) = &dirs.jump {
                    mh.direction = parse_direction_sign(jump);
                }
            }

            // Per-track bar IDs
            let bar_ids = parse_ids(&mb.bars);
            for (t_idx, &bar_id) in bar_ids.iter().enumerate() {
                if t_idx < num_tracks {
                    track_bar_ids[t_idx].push(bar_id);
                }
            }

            self.measure_headers.push(mh);
        }

        let num_measures = self.measure_headers.len();

        // 5. Tracks
        self.tracks.clear();

        for (t_idx, g_track) in gpif.tracks.tracks.iter().enumerate() {
            let mut track = SongTrack {
                name: g_track.name.clone(),
                short_name: g_track.short_name.clone(),
                number: (t_idx + 1) as i32,
                ..Default::default()
            };

            // Color
            if let Some(color_str) = &g_track.color {
                let rgb: Vec<i32> = color_str
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();
                if rgb.len() == 3 {
                    track.color = rgb[0] * 65536 + rgb[1] * 256 + rgb[2];
                }
            }

            // Tuning: GP6 track-level properties, GP7 staves
            if let Some(props) = &g_track.properties {
                track.strings = extract_tuning(&props.properties);
            }
            if track.strings.is_empty() {
                if let Some(staves) = &g_track.staves {
                    for staff in &staves.staves {
                        if let Some(props) = &staff.properties {
                            track.strings = extract_tuning(&props.properties);
                            if !track.strings.is_empty() {
                                break;
                            }
                        }
                    }
                }
            }
            if track.strings.is_empty() {
                track.strings = vec![(1, 64), (2, 59), (3, 55), (4, 50), (5, 45), (6, 40)];
            }

            track.fret_count = 24;

            // MIDI
            if let Some(gm) = &g_track.general_midi {
                if let Some(ch) = gm.primary_channel {
                    track.channel_index = ch as usize;
                    track.percussion_track = ch == 9;
                }
                track.midi_program_gpif = gm.program;
                if let Some(port) = gm.port {
                    track.port = port as u8;
                }
            }

            // Transpose
            if let Some(tr) = &g_track.transpose {
                track.transpose_chromatic = tr.chromatic.unwrap_or(0);
                track.transpose_octave = tr.octave.unwrap_or(0);
            }

            // Current dynamic (persists across beats)
            let mut current_velocity: i16 = FORTE;

            // RSE (GPX)
            if let Some(rse_wrapper) = &g_track.rse {
                // Populate humanize/instrument from GPX RSE if possible
                // Currently just setting default or mapping names if available
                if let Some(chains) = &rse_wrapper.effect_chains {
                    if let Some(first_chain) = chains.effect_chains.first() {
                        track.rse.instrument.effect_category = first_chain.name.clone();
                    }
                }
            }

            // Measures
            for m_idx in 0..num_measures {
                let mut measure = Measure {
                    number: m_idx + 1,
                    track_index: t_idx,
                    ..Default::default()
                };

                if m_idx < self.measure_headers.len() {
                    measure.time_signature = self.measure_headers[m_idx].time_signature.clone();
                    measure.key_signature = self.measure_headers[m_idx].key_signature.clone();
                }

                let bar_id = if m_idx < track_bar_ids[t_idx].len() {
                    track_bar_ids[t_idx][m_idx]
                } else {
                    -1
                };

                if let Some(bar) = bars_map.get(&bar_id) {
                    measure.simile_mark = bar.simile_mark.clone();
                    let voice_ids = parse_ids(&bar.voices);
                    measure.voices.clear();

                    for &vid in &voice_ids {
                        if vid < 0 {
                            continue;
                        }
                        let mut s_voice = SongVoice::default();

                        if let Some(g_voice) = voices_map.get(&vid) {
                            let beat_ids = parse_ids(&g_voice.beats);

                            for &bid in &beat_ids {
                                if let Some(g_beat) = beats_map.get(&bid) {
                                    let s_beat = convert_beat(
                                        g_beat,
                                        &rhythms_map,
                                        &notes_map,
                                        &mut current_velocity,
                                    );
                                    s_voice.beats.push(s_beat);
                                }
                            }
                        }
                        measure.voices.push(s_voice);
                    }
                }
                track.measures.push(measure);
            }
            self.tracks.push(track);
        }
    }
}

fn convert_beat(
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
                    if let Some(val) = bp.float {
                        if val != 0.0 {
                            s_beat.effect.tremolo_bar = Some(build_bend_effect(0.0, val));
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
                }
            }
        }
        None => {
            s_beat.status = BeatStatus::Rest;
        }
    }

    s_beat
}

fn convert_note(
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
    let mut bend_middle_offset2: Option<f64> = None;
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
                bend_middle_offset2 = prop.float;
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
            "Tapped" => {
                if prop.enable.is_some() {
                    s_note.effect.harmonic = Some(HarmonicEffect {
                        kind: HarmonicType::Tapped,
                        ..Default::default()
                    });
                }
            }
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
                bend_middle_offset2,
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
