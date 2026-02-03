use fraction::ToPrimitive;

use crate::error::{GpError, GpResult};
use crate::io::primitive::*;
use crate::model::key_signature::*;
use crate::model::song::Song;
use crate::traits::effect_ops::SongEffectOps;
use crate::types::enums::*;
use crate::types::note::Note;

/// Read note effects. First byte is note effects flags
pub fn read_note_effects_v3(
    song: &Song,
    data: &[u8],
    seek: &mut usize,
    note: &mut Note,
) -> GpResult<()> {
    let flags = read_byte(data, seek)?;
    //println!("read_effect(), flags: {}", flags);
    note.effect.hammer = (flags & 0x02) == 0x02;
    note.effect.let_ring = (flags & 0x08) == 0x08;
    if (flags & 0x01) == 0x01 {
        note.effect.bend = song.read_bend_effect(data, seek)?;
    }
    if (flags & 0x10) == 0x10 {
        note.effect.grace = Some(song.read_grace_effect(data, seek)?);
    }
    if (flags & 0x04) == 0x04 {
        note.effect.slides.push(SlideType::ShiftSlideTo);
    }
    //println!("read_note_effects(): {:?}", note);
    Ok(())
}

/// Read note effects. The effects presence for the current note is set by the 2 bytes of flags.
pub fn read_note_effects_v4(
    song: &mut Song,
    data: &[u8],
    seek: &mut usize,
    note: &mut Note,
) -> GpResult<()> {
    let flags1 = read_signed_byte(data, seek)?;
    let flags2 = read_signed_byte(data, seek)?;
    note.effect.hammer = (flags1 & 0x02) == 0x02;
    note.effect.let_ring = (flags1 & 0x08) == 0x08;
    note.effect.staccato = (flags2 & 0x01) == 0x01;
    note.effect.palm_mute = (flags2 & 0x02) == 0x02;
    note.effect.vibrato = (flags2 & 0x40) == 0x40 || note.effect.vibrato;
    if (flags1 & 0x01) == 0x01 {
        note.effect.bend = song.read_bend_effect(data, seek)?;
    }
    if (flags1 & 0x10) == 0x10 {
        if song.version.number >= (5, 0, 0) {
            note.effect.grace = Some(song.read_grace_effect_v5(data, seek)?);
        } else {
            note.effect.grace = Some(song.read_grace_effect(data, seek)?);
        }
    }
    if (flags2 & 0x04) == 0x04 {
        note.effect.tremolo_picking = Some(song.read_tremolo_picking(data, seek)?);
    }
    if (flags2 & 0x08) == 0x08 {
        if song.version.number >= (5, 0, 0) {
            note.effect.slides.extend(song.read_slides_v5(data, seek)?);
        } else {
            note.effect
                .slides
                .push(get_slide_type(read_signed_byte(data, seek)?)?);
        }
    }
    if (flags2 & 0x10) == 0x10 {
        if song.version.number >= (5, 0, 0) {
            note.effect.harmonic = Some(song.read_harmonic_v5(data, seek)?);
        } else {
            note.effect.harmonic = Some(song.read_harmonic(data, seek, note)?);
        }
    }
    if (flags2 & 0x20) == 0x20 {
        note.effect.trill = Some(song.read_trill(data, seek)?);
    }
    Ok(())
}

pub fn write_note_effects_v3(song: &Song, data: &mut Vec<u8>, note: &Note) {
    let mut flags1 = 0u8;
    if note.effect.is_bend() {
        flags1 |= 0x01;
    }
    if note.effect.hammer {
        flags1 |= 0x02;
    }
    if note.effect.slides.contains(&SlideType::ShiftSlideTo)
        || note.effect.slides.contains(&SlideType::LegatoSlideTo)
    {
        flags1 |= 0x04;
    }
    if note.effect.let_ring {
        flags1 |= 0x08;
    }
    if note.effect.is_grace() {
        flags1 |= 0x10;
    }
    write_byte(data, flags1);
    if (flags1 & 0x01) == 0x01 {
        song.write_bend(data, &note.effect.bend);
    }
    if (flags1 & 0x10) == 0x10 {
        song.write_grace(data, &note.effect.grace);
    }
}

pub fn write_note_effects(
    song: &Song,
    data: &mut Vec<u8>,
    note: &Note,
    strings: &[(i8, i8)],
    version: &(u8, u8, u8),
) -> GpResult<()> {
    let mut flags1 = 0i8;
    if note.effect.is_bend() {
        flags1 |= 0x01;
    }
    if note.effect.hammer {
        flags1 |= 0x02;
    }
    if note.effect.let_ring {
        flags1 |= 0x08;
    }
    if note.effect.is_grace() {
        flags1 |= 0x10;
    }
    write_signed_byte(data, flags1);

    let mut flags2 = 0i8;
    if note.effect.staccato {
        flags2 |= 0x01;
    }
    if note.effect.palm_mute {
        flags2 |= 0x01;
    }
    if note.effect.is_tremollo_picking() {
        flags2 |= 0x01;
    }
    if !note.effect.slides.is_empty() {
        flags2 |= 0x01;
    }
    if note.effect.is_harmonic() {
        flags2 |= 0x01;
    }
    if note.effect.is_trill() {
        flags2 |= 0x01;
    }
    if note.effect.vibrato {
        flags2 |= 0x01;
    }
    write_signed_byte(data, flags2);

    if (flags1 & 0x01) == 0x01 {
        song.write_bend(data, &note.effect.bend);
    }
    if (flags1 & 0x10) == 0x10 {
        if version.0 < 5 {
            song.write_grace(data, &note.effect.grace);
        } else {
            song.write_grace_v5(data, &note.effect.grace);
        }
    }
    if (flags2 & 0x04) == 0x04 {
        if let Some(tp) = &note.effect.tremolo_picking {
            let val = match tp.duration.value.to_u8().unwrap() {
                DURATION_EIGHTH => 1,
                DURATION_SIXTEENTH => 2,
                DURATION_THIRTY_SECOND => 3,
                _ => {
                    return Err(GpError::InvalidValue {
                        context: "tremolo picking",
                        value: tp.duration.value.to_i64().unwrap_or(0),
                    })
                }
            };
            write_signed_byte(data, val);
        }
    }
    if (flags2 & 0x08) == 0x08 {
        if version.0 < 5 {
            write_signed_byte(data, from_slide_type(&note.effect.slides[0]));
        } else {
            song.write_slides_v5(data, &note.effect.slides);
        }
    }
    if (flags2 & 0x10) == 0x10 {
        if version.0 < 5 {
            song.write_harmonic(data, note, strings)?;
        } else {
            song.write_harmonic_v5(data, note, strings)?;
        }
    }
    if (flags2 & 0x20) == 0x20 {
        //trill
        if let Some(t) = &note.effect.trill {
            write_signed_byte(data, t.fret);
            let val = match t.duration.value.to_u8().unwrap() {
                DURATION_SIXTEENTH => 1,
                DURATION_THIRTY_SECOND => 2,
                DURATION_SIXTY_FOURTH => 3,
                _ => {
                    return Err(GpError::InvalidValue {
                        context: "trill duration",
                        value: t.duration.value.to_i64().unwrap_or(0),
                    })
                }
            };
            write_signed_byte(data, val);
        } else {
            return Err(GpError::InvalidValue {
                context: "trill",
                value: 0,
            });
        }
    }
    Ok(())
}
