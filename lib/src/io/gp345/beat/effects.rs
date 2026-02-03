use fraction::ToPrimitive;

use crate::error::GpResult;
use crate::io::primitive::*;
use crate::model::key_signature::*;
use crate::model::note::NoteEffect;
use crate::model::song::Song;
use crate::traits::effect_ops::SongEffectOps;
use crate::types::beat::{Beat, BeatEffects, BeatStroke};
use crate::types::effects::bend::*;
use crate::types::effects::*;
use crate::types::enums::*;

/// Read beat effects. The first byte is effects flags:
/// - *0x01*: vibrato
/// - *0x02*: wide vibrato
/// - *0x04*: natural harmonic
/// - *0x08*: artificial harmonic
/// - *0x10*: fade in
/// - *0x20*: tremolo bar or slap effect
/// - *0x40*: beat stroke direction
/// - *0x80*: *blank*
/// - Tremolo bar or slap effect: `byte`. If it's 0 then tremolo bar should be read (see `TremoloBar::read()`). Else it's tapping and values of the byte map to:
/// - *1*: tap
/// - *2*: slap
/// - *3*: pop
/// - Beat stroke direction. See `BeatStroke::read()`
pub fn read_beat_effects_v3(
    song: &Song,
    data: &[u8],
    seek: &mut usize,
    note_effect: &mut NoteEffect,
) -> GpResult<BeatEffects> {
    //println!("read_beat_effects()");
    let mut be = BeatEffects::default();
    let flags = read_byte(data, seek)?;
    note_effect.vibrato = (flags & 0x01) == 0x01 || note_effect.vibrato;
    be.vibrato = (flags & 0x02) == 0x02 || be.vibrato;
    be.fade_in = (flags & 0x10) == 0x10;
    if (flags & 0x20) == 0x20 {
        be.slap_effect = get_slap_effect(read_byte(data, seek)?)?;
        if be.slap_effect == SlapEffect::None {
            be.tremolo_bar = Some(read_tremolo_bar(data, seek)?);
        } else {
            read_int(data, seek)?;
        }
    }
    if (flags & 0x40) == 0x40 {
        be.stroke = read_beat_stroke(song, data, seek)?;
    }
    //In GP3 harmonics apply to the whole beat, not the individual notes. Here we set the noteEffect for all the notes in the beat.
    if (flags & 0x04) == 0x04 {
        note_effect.harmonic = Some(HarmonicEffect::default());
    }
    if (flags & 0x08) == 0x08 {
        note_effect.harmonic = Some(HarmonicEffect {
            kind: HarmonicType::Artificial,
            ..Default::default()
        });
    }
    Ok(be)
}

///Read beat effects. Beat effects are read using two byte flags. The first byte of flags is:
/// - *0x01*: *blank*
/// - *0x02*: wide vibrato
/// - *0x04*: *blank*
/// - *0x08*: *blank*
/// - *0x10*: fade in
/// - *0x20*: slap effect
/// - *0x40*: beat stroke
/// - *0x80*: *blank*
///
/// The second byte of flags is:
/// - *0x01*: rasgueado
/// - *0x02*: pick stroke
/// - *0x04*: tremolo bar
/// - *0x08*: *blank*
/// - *0x10*: *blank*
/// - *0x20*: *blank*
/// - *0x40*: *blank*
/// - *0x80*: *blank*
///
/// Flags are followed by:
/// - Slap effect: `signed-byte`. For value mapping see `SlapEffect`.
/// - Tremolo bar. See `readTremoloBar`.
/// - Beat stroke. See `readBeatStroke`.
/// - Pick stroke: `signed-byte`. For value mapping see `BeatStrokeDirection`.
pub fn read_beat_effects_v4(song: &Song, data: &[u8], seek: &mut usize) -> GpResult<BeatEffects> {
    let mut be = BeatEffects::default();
    let flags1 = read_signed_byte(data, seek)?;
    let flags2 = read_signed_byte(data, seek)?;
    be.vibrato = (flags1 & 0x02) == 0x02 || be.vibrato;
    be.fade_in = (flags1 & 0x10) == 0x10;
    if (flags1 & 0x20) == 0x20 {
        be.slap_effect = get_slap_effect(read_signed_byte(data, seek)?.to_u8().unwrap())?;
    }
    if (flags2 & 0x04) == 0x04 {
        be.tremolo_bar = song.read_bend_effect(data, seek)?;
    }
    if (flags1 & 0x40) == 0x40 {
        be.stroke = read_beat_stroke(song, data, seek)?;
    }
    be.has_rasgueado = (flags2 & 0x01) == 0x01;
    if (flags2 & 0x02) == 0x02 {
        be.pick_stroke = get_beat_stroke_direction(read_signed_byte(data, seek)?)?;
    }
    //println!("Beat effect: {:?}", be);
    Ok(be)
}

/// Read beat stroke. Beat stroke consists of two `Bytes <byte>` which correspond to stroke up
/// and stroke down speed. See `BeatStrokeDirection` for value mapping.
pub fn read_beat_stroke(song: &Song, data: &[u8], seek: &mut usize) -> GpResult<BeatStroke> {
    //println!("read_beat_stroke()");
    let mut bs = BeatStroke::default();
    let down = read_signed_byte(data, seek)?;
    let up = read_signed_byte(data, seek)?;
    if up > 0 {
        bs.direction = BeatStrokeDirection::Up;
        bs.value = stroke_value(up).to_u16().unwrap();
    }
    if down > 0 {
        bs.direction = BeatStrokeDirection::Down;
        bs.value = stroke_value(down).to_u16().unwrap();
    }
    if song.version.number >= (5, 0, 0) {
        bs.swap_direction();
    }
    Ok(bs)
}

pub fn stroke_value(value: i8) -> u8 {
    match value {
        1 => DURATION_HUNDRED_TWENTY_EIGHTH,
        2 => DURATION_SIXTY_FOURTH,
        3 => DURATION_THIRTY_SECOND,
        4 => DURATION_SIXTEENTH,
        5 => DURATION_EIGHTH,
        6 => DURATION_QUARTER,
        _ => DURATION_SIXTY_FOURTH,
    }
}

/// Read tremolo bar beat effect. The only type of tremolo bar effect Guitar Pro 3 supports is `dip <BendType::Dip>`. The value of the
/// effect is encoded in `Int` and shows how deep tremolo bar is pressed.
pub fn read_tremolo_bar(data: &[u8], seek: &mut usize) -> GpResult<BendEffect> {
    //println!("read_tremolo_bar()");
    let mut be = BendEffect {
        kind: BendType::Dip,
        ..Default::default()
    };
    be.value = read_int(data, seek)?.to_i16().unwrap();
    be.points.push(BendPoint {
        position: 0,
        value: 0,
        ..Default::default()
    });
    be.points.push(BendPoint {
        position: BEND_EFFECT_MAX_POSITION / 2,
        value: (-f32::from(be.value) / GP_BEND_SEMITONE)
            .round()
            .to_i8()
            .unwrap(),
        ..Default::default()
    });
    be.points.push(BendPoint {
        position: BEND_EFFECT_MAX_POSITION,
        value: 0,
        ..Default::default()
    });
    Ok(be)
}

pub fn write_beat_effect_v3(data: &mut Vec<u8>, beat: &Beat) {
    let mut flags1: u8 = 0;
    if beat.has_vibrato() {
        flags1 |= 0x01;
    }
    if beat.effect.vibrato {
        flags1 |= 0x01;
    }
    if beat.has_harmonic() {
        for n in 0..beat.notes.len() {
            if let Some(h) = &beat.notes[n].effect.harmonic {
                if h.kind == HarmonicType::Natural {
                    flags1 |= 0x04;
                }
                if h.kind == HarmonicType::Artificial {
                    flags1 |= 0x08;
                }
            }
        }
    }
    if beat.effect.fade_in {
        flags1 |= 0x10;
    }
    if beat.effect.is_tremolo_bar() || beat.effect.is_slap_effect() {
        flags1 |= 0x20;
    }
    if beat.effect.stroke.direction != BeatStrokeDirection::None && beat.effect.stroke.value != 0 {
        flags1 |= 0x40;
    }
    write_byte(data, flags1);
    if (flags1 & 0x20) == 0x20 {
        write_byte(data, from_slap_effect(&beat.effect.slap_effect));
        write_tremolo_bar(data, &beat.effect.tremolo_bar);
    }
    if (flags1 & 0x40) == 0x40 {
        write_beat_stroke(data, &beat.effect.stroke, &(3, 0, 0));
    }
}

pub fn write_beat_effect_v4(song: &Song, data: &mut Vec<u8>, beat: &Beat, version: &(u8, u8, u8)) {
    let mut flags1: i8 = 0;
    if beat.has_vibrato() {
        flags1 |= 0x01;
    }
    if beat.effect.fade_in {
        flags1 |= 0x10;
    }
    if beat.effect.is_slap_effect() {
        flags1 |= 0x20;
    }
    if beat.effect.stroke.direction != BeatStrokeDirection::None && beat.effect.stroke.value != 0 {
        flags1 |= 0x40;
    }
    write_signed_byte(data, flags1);

    let mut flags2 = 0i8;
    if beat.effect.has_rasgueado {
        flags2 |= 0x01;
    }
    if beat.effect.has_pick_stroke() {
        flags2 |= 0x02;
    }
    if beat.effect.is_tremolo_bar() {
        flags2 |= 0x04;
    }
    write_signed_byte(data, flags2);

    if (flags1 & 0x20) == 0x20 {
        write_signed_byte(
            data,
            from_slap_effect(&beat.effect.slap_effect).to_i8().unwrap(),
        );
    }
    if (flags2 & 0x04) == 0x04 {
        song.write_bend(data, &beat.effect.tremolo_bar);
    } //write tremolo bar
    if (flags2 & 0x40) == 0x40 {
        write_beat_stroke(data, &beat.effect.stroke, version);
    }
    if (flags2 & 0x02) == 0x02 {
        write_signed_byte(data, from_beat_stroke_direction(&beat.effect.pick_stroke));
    }
}

pub fn write_tremolo_bar(data: &mut Vec<u8>, bar: &Option<BendEffect>) {
    if let Some(b) = bar {
        write_i32(data, b.value.to_i32().unwrap());
    } else {
        write_i32(data, 0);
    }
}

pub fn write_beat_stroke(data: &mut Vec<u8>, stroke: &BeatStroke, version: &(u8, u8, u8)) {
    let mut stroke = stroke.clone();
    if version.0 == 5 {
        stroke.swap_direction();
    }
    let mut stroke_down = 0i8;
    let mut stroke_up = 0i8;
    if stroke.direction == BeatStrokeDirection::Up {
        stroke_up = from_stroke_value(stroke.value.to_u8().unwrap());
    } else if stroke.direction == BeatStrokeDirection::Down {
        stroke_down = from_stroke_value(stroke.value.to_u8().unwrap());
    }
    write_signed_byte(data, stroke_down);
    write_signed_byte(data, stroke_up);
}

pub fn from_stroke_value(value: u8) -> i8 {
    if value == DURATION_HUNDRED_TWENTY_EIGHTH {
        1
    } else if value == DURATION_SIXTY_FOURTH {
        2
    } else if value == DURATION_THIRTY_SECOND {
        3
    } else if value == DURATION_SIXTEENTH {
        4
    } else if value == DURATION_EIGHTH {
        5
    } else if value == DURATION_QUARTER {
        6
    } else {
        1
    }
}
