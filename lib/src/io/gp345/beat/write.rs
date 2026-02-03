use crate::error::GpResult;
use crate::io::primitive::*;
use crate::model::chord::SongChordOps;
use crate::model::mix_table::SongMixTableOps;
use crate::model::note::SongNoteOps;
use crate::model::song::Song;
use crate::types::beat::Beat;
use crate::types::enums::*;

use super::effects::*;

pub fn write_beat_v3(song: &Song, data: &mut Vec<u8>, beat: &Beat) -> GpResult<()> {
    let mut flags = 0u8;
    if beat.duration.dotted {
        flags |= 0x01;
    }
    if beat.effect.is_chord() {
        flags |= 0x02;
    }
    if !beat.text.is_empty() {
        flags |= 0x04;
    }
    if beat.effect.is_default() {
        flags |= 0x08;
    }
    if let Some(mtc) = &beat.effect.mix_table_change {
        if mtc.is_just_wah() {
            flags |= 0x10;
        }
    }
    if !beat.duration.is_default_tuplet() {
        flags |= 0x20;
    }
    if beat.status != BeatStatus::Normal {
        flags |= 0x40;
    }
    write_byte(data, flags);
    if (flags & 0x40) == 0x40 {
        write_byte(data, from_beat_status(&beat.status));
    }
    beat.duration.write_duration(data, flags);
    if (flags & 0x02) == 0x02 {
        song.write_chord(data, beat);
    }
    if (flags & 0x04) == 0x04 {
        write_int_byte_size_string(data, &beat.text);
    }
    if (flags & 0x08) == 0x08 {
        write_beat_effect_v3(data, beat);
    }
    if (flags & 0x10) == 0x10 {
        song.write_mix_table_change(data, &beat.effect.mix_table_change, &(3, 0, 0));
    }
    song.write_notes(data, beat, &Vec::new(), &(3, 0, 0))?;
    Ok(())
}

pub fn write_beat(
    song: &Song,
    data: &mut Vec<u8>,
    beat: &Beat,
    strings: &[(i8, i8)],
    version: &(u8, u8, u8),
) -> GpResult<()> {
    let mut flags = 0u8;
    if beat.duration.dotted {
        flags |= 0x01;
    }
    if beat.effect.is_chord() {
        flags |= 0x02;
    }
    if !beat.text.is_empty() {
        flags |= 0x04;
    }
    if beat.effect.is_default() {
        flags |= 0x08;
    }
    if let Some(mtc) = &beat.effect.mix_table_change {
        if mtc.is_just_wah() || version.0 > 4 {
            flags |= 0x10;
        }
    }
    if !beat.duration.is_default_tuplet() {
        flags |= 0x20;
    }
    if beat.status != BeatStatus::Normal {
        flags |= 0x40;
    }
    write_byte(data, flags);
    if (flags & 0x40) == 0x40 {
        write_byte(data, from_beat_status(&beat.status));
    }
    beat.duration.write_duration(data, flags);
    if (flags & 0x02) == 0x02 {
        song.write_chord_v4(data, beat);
    }
    if (flags & 0x04) == 0x04 {
        write_int_byte_size_string(data, &beat.text);
    }
    if (flags & 0x08) == 0x08 {
        write_beat_effect_v4(song, data, beat, version);
    }
    if (flags & 0x10) == 0x10 {
        song.write_mix_table_change(data, &beat.effect.mix_table_change, version);
    }
    song.write_notes(data, beat, strings, version)?;
    if version.0 == 5 {
        let mut flags2 = 0i16;
        if beat.display.break_beam {
            flags2 |= 0x0001;
        }
        if beat.display.beam_direction == VoiceDirection::Down {
            flags2 |= 0x0002;
        }
        if beat.display.force_beam {
            flags2 |= 0x0004;
        }
        if beat.display.beam_direction == VoiceDirection::Up {
            flags2 |= 0x0008;
        }
        if beat.octave == Octave::Ottava {
            flags2 |= 0x0010;
        }
        if beat.octave == Octave::OttavaBassa {
            flags2 |= 0x0020;
        }
        if beat.octave == Octave::Quindicesima {
            flags2 |= 0x0040;
        }
        if beat.octave == Octave::QuindicesimaBassa {
            flags2 |= 0x0100;
        }
        if beat.display.tuplet_bracket == TupletBracket::Start {
            flags2 |= 0x0200;
        }
        if beat.display.tuplet_bracket == TupletBracket::End {
            flags2 |= 0x0400;
        }
        if beat.display.break_secondary != 0 {
            flags2 |= 0x0800;
        }
        if beat.display.break_secondary_tuplet {
            flags2 |= 0x1000;
        }
        if beat.display.force_bracket {
            flags2 |= 0x2000;
        }
        write_i16(data, flags2);
        if (flags2 & 0x0800) == 0x0800 {
            write_byte(data, beat.display.break_secondary);
        }
    }
    Ok(())
}
