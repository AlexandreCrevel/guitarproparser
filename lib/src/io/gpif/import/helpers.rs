// GPIF import helper functions
use crate::io::gpif::{Gpif, Property};
use crate::model::effects::{HarmonicEffect, FORTE, MIN_VELOCITY, VELOCITY_INCREMENT};
use crate::model::enums::*;
use crate::model::headers::FermataType;

/// Convert GPIF note value string to Duration.value.
/// Falls back to Quarter (4) for unknown values.
pub(crate) fn note_value_to_duration(s: &str) -> u16 {
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

/// Convert GPIF grace note rhythm NoteValue to the in-memory grace duration value.
/// The in-memory representation matches standard note values: 16 = sixteenth, 32 = thirty-second, 64 = sixty-fourth.
/// (The GP5 binary format stores these as `1 << (7 - byte)`, so byte 3 = 16th, byte 2 = 32nd, byte 1 = 64th.)
pub(crate) fn grace_note_value_to_duration(s: &str) -> u8 {
    match s {
        "16th" => 16,
        "32nd" => 32,
        "64th" => 64,
        _ => 16, // Default to sixteenth for unrecognized values
    }
}

/// Convert GPIF dynamic string to MIDI velocity
pub(crate) fn dynamic_to_velocity(s: &str) -> i16 {
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
pub(crate) fn parse_ids(s: &str) -> Vec<i32> {
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
pub(crate) fn parse_slide_flags(flags: i32) -> Vec<SlideType> {
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
pub(crate) fn parse_harmonic_type(htype: &str) -> HarmonicEffect {
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
pub(crate) fn parse_direction_sign(s: &str) -> Option<DirectionSign> {
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

/// Extract tuning pitches from a property list.
pub(crate) fn extract_tuning(properties: &[Property]) -> Vec<(i8, i8)> {
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
pub(crate) fn parse_fingering(s: &str) -> Fingering {
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
pub(crate) fn parse_fermata_type(s: &str) -> FermataType {
    match s {
        "Short" => FermataType::Short,
        "Long" => FermataType::Long,
        _ => FermataType::Medium,
    }
}

/// Parse a fraction string like "0/1" or "2/1" into a (numerator, denominator) tuple.
pub(crate) fn parse_fraction_offset(s: &str) -> (i32, i32) {
    let parts: Vec<&str> = s.split('/').collect();
    if parts.len() == 2 {
        let num = parts[0].parse::<i32>().unwrap_or(0);
        let den = parts[1].parse::<i32>().unwrap_or(1).max(1);
        (num, den)
    } else {
        (0, 1)
    }
}

/// Parse GPIF version string (e.g. "7") into a version tuple.
/// Falls back to the provided default if parsing fails.
pub(crate) fn parse_gpif_version(gpif: &Gpif, default: (u8, u8, u8)) -> (u8, u8, u8) {
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
