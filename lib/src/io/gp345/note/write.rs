use fraction::ToPrimitive;

use crate::error::GpResult;
use crate::io::primitive::*;
use crate::model::song::Song;
use crate::types::beat::Beat;
use crate::types::enums::*;
use crate::types::note::Note;
use crate::types::{pack_velocity, DEFAULT_VELOCITY};

use super::effects::*;

pub fn write_notes(
    song: &Song,
    data: &mut Vec<u8>,
    beat: &Beat,
    strings: &[(i8, i8)],
    version: &(u8, u8, u8),
) -> GpResult<()> {
    let mut string_flags: u8 = 0;
    for i in 0..beat.notes.len() {
        string_flags |= 1 << (7 - beat.notes[i].string);
    }
    write_byte(data, string_flags);
    let mut notes = beat.notes.clone();
    notes.sort_by_key(|k| k.string);
    for note in &notes {
        if version.0 == 3 {
            write_note_v3(song, data, note)?;
        } else if version.0 == 4 {
            write_note_v4(song, data, note, strings, version)?;
        } else if version.0 == 5 {
            write_note_v5(song, data, note, strings, version)?;
        }
    }
    Ok(())
}

pub fn write_note_v3(song: &Song, data: &mut Vec<u8>, note: &Note) -> GpResult<()> {
    let flags: u8 = pack_note_flags(note, &(3, 0, 0));
    write_byte(data, flags);
    if (flags & 0x20) == 0x20 {
        write_byte(data, from_note_type(&note.kind));
    }
    if (flags & 0x01) == 0x01 {
        write_signed_byte(data, note.duration.unwrap());
        write_signed_byte(data, note.tuplet.unwrap());
    }
    if (flags & 0x10) == 0x10 {
        write_signed_byte(data, pack_velocity(note.velocity));
    }
    if (flags & 0x20) == 0x20 {
        if note.kind != NoteType::Rest {
            write_signed_byte(data, note.value.to_i8().unwrap());
        } else {
            write_signed_byte(data, 0);
        }
    }
    if (flags & 0x08) == 0x08 {
        write_note_effects_v3(song, data, note);
    }
    Ok(())
}

pub fn write_note_v4(
    song: &Song,
    data: &mut Vec<u8>,
    note: &Note,
    strings: &[(i8, i8)],
    version: &(u8, u8, u8),
) -> GpResult<()> {
    let flags: u8 = pack_note_flags(note, version);
    write_byte(data, flags);
    if (flags & 0x20) == 0x20 {
        write_byte(data, from_note_type(&note.kind));
    }
    if (flags & 0x01) == 0x01 {
        write_signed_byte(data, note.duration.unwrap());
        write_signed_byte(data, note.tuplet.unwrap());
    }
    if (flags & 0x10) == 0x10 {
        write_signed_byte(data, pack_velocity(note.velocity));
    }
    if (flags & 0x20) == 0x20 {
        if note.kind != NoteType::Rest {
            write_signed_byte(data, note.value.to_i8().unwrap());
        } else {
            write_signed_byte(data, 0);
        }
    }
    if (flags & 0x80) == 0x80 {
        write_signed_byte(data, from_fingering(&note.effect.left_hand_finger));
        write_signed_byte(data, from_fingering(&note.effect.right_hand_finger));
    }
    if (flags & 0x08) == 0x08 {
        if version.0 == 3 {
            write_note_effects_v3(song, data, note);
        } else {
            write_note_effects(song, data, note, strings, version)?;
        }
    }
    Ok(())
}

pub fn write_note_v5(
    song: &Song,
    data: &mut Vec<u8>,
    note: &Note,
    strings: &[(i8, i8)],
    version: &(u8, u8, u8),
) -> GpResult<()> {
    let flags: u8 = pack_note_flags(note, version);
    write_byte(data, flags);
    if (flags & 0x20) == 0x20 {
        write_byte(data, from_note_type(&note.kind));
    }
    if (flags & 0x10) == 0x10 {
        write_signed_byte(data, pack_velocity(note.velocity));
    }
    if (flags & 0x20) == 0x20 {
        if note.kind != NoteType::Tie {
            write_signed_byte(data, note.value.to_i8().unwrap());
        } else {
            write_signed_byte(data, 0);
        }
    }
    if (flags & 0x80) == 0x80 {
        write_signed_byte(data, from_fingering(&note.effect.left_hand_finger));
        write_signed_byte(data, from_fingering(&note.effect.right_hand_finger));
    }
    if (flags & 0x01) == 0x01 {
        write_f64(data, note.duration_percent.to_f64().unwrap());
    }
    let mut flags2 = 0u8;
    if note.swap_accidentals {
        flags2 |= 0x02;
    }
    write_byte(data, flags2);
    if (flags & 0x08) == 0x08 {
        write_note_effects(song, data, note, strings, version)?;
    }
    Ok(())
}

pub fn pack_note_flags(note: &Note, version: &(u8, u8, u8)) -> u8 {
    let mut flags: u8 = 0u8;
    if note.duration.is_some() && note.tuplet.is_some() {
        flags |= 0x01;
    }
    if note.effect.heavy_accentuated_note {
        flags |= 0x02;
    }
    if note.effect.ghost_note {
        flags |= 0x04;
    }
    if note.effect.is_default() {
        flags |= 0x08;
    }
    if note.velocity != DEFAULT_VELOCITY {
        flags |= 0x10;
    }
    flags |= 0x20;
    if version.0 > 3 {
        if note.effect.accentuated_note {
            flags |= 0x40;
        }
        if note.effect.is_fingering() {
            flags |= 0x80;
        }
    }
    if version.0 >= 5 && (note.duration_percent - 1.0).abs() > 1e-3 {
        flags |= 0x01;
    }
    flags
}
