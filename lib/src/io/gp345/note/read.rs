use fraction::ToPrimitive;

use crate::error::GpResult;
use crate::io::primitive::*;
use crate::model::key_signature::*;
use crate::model::song::Song;
use crate::types::beat::Beat;
use crate::types::effects::*;
use crate::types::enums::*;
use crate::types::note::{Note, NoteEffect};

use super::effects::*;

/// Read notes. First byte lists played strings
pub fn read_notes(
    song: &mut Song,
    data: &[u8],
    seek: &mut usize,
    track_index: usize,
    beat: &mut Beat,
    duration: &Duration,
    note_effect: NoteEffect,
) -> GpResult<()> {
    let flags = read_byte(data, seek)?;
    //println!("read_notes(), flags: {}", flags);
    for i in 0..song.tracks[track_index].strings.len() {
        if (flags & 1 << (7 - song.tracks[track_index].strings[i].0)) > 0 {
            let mut note = Note {
                effect: note_effect.clone(),
                ..Default::default()
            };
            if song.version.number < (5, 0, 0) {
                read_note(
                    song,
                    data,
                    seek,
                    &mut note,
                    song.tracks[track_index].strings[i],
                    track_index,
                )?;
            } else {
                read_note_v5(
                    song,
                    data,
                    seek,
                    &mut note,
                    song.tracks[track_index].strings[i],
                    track_index,
                )?;
            }
            beat.notes.push(note);
        }
        beat.duration = duration.clone();
    }
    Ok(())
}

/// Read note. The first byte is note flags
pub fn read_note(
    song: &mut Song,
    data: &[u8],
    seek: &mut usize,
    note: &mut Note,
    guitar_string: (i8, i8),
    track_index: usize,
) -> GpResult<()> {
    let flags = read_byte(data, seek)?;
    note.string = guitar_string.0;
    note.effect.ghost_note = (flags & 0x04) == 0x04;
    //println!("read_note(), flags: {} \t string: {} \t ghost note: {}", flags, guitar_string.0, note.effect.ghost_note);
    if (flags & 0x20) == 0x20 {
        note.kind = get_note_type(read_byte(data, seek)?);
    }
    if (flags & 0x01) == 0x01 {
        //println!("read_note(), duration: {} \t tuplet: {}",duration, tuplet);
        note.duration = Some(read_signed_byte(data, seek)?);
        note.tuplet = Some(read_signed_byte(data, seek)?);
    }
    if (flags & 0x10) == 0x10 {
        let v = read_signed_byte(data, seek)?;
        //println!("read_note(), v: {}", v);
        note.velocity = unpack_velocity(v.to_i16().unwrap());
        //println!("read_note(), velocity: {}", note.velocity);
    }
    if (flags & 0x20) == 0x20 {
        let fret = read_signed_byte(data, seek)?;
        let value = if note.kind == NoteType::Tie {
            get_tied_note_value(song, guitar_string.0, track_index)
        } else {
            fret.to_i16().unwrap()
        };
        note.value = value.clamp(0, 99);
        //println!("read_note(), value: {}", note.value);
    }
    if (flags & 0x80) == 0x80 {
        note.effect.left_hand_finger = get_fingering(read_signed_byte(data, seek)?);
        note.effect.right_hand_finger = get_fingering(read_signed_byte(data, seek)?);
    }
    if (flags & 0x08) == 0x08 {
        if song.version.number == (3, 0, 0) {
            read_note_effects_v3(song, data, seek, note)?;
        } else if song.version.number.0 == 4 {
            read_note_effects_v4(song, data, seek, note)?;
        }
        if note.effect.is_harmonic() && note.effect.harmonic.is_some() {
            let mut h = note.effect.harmonic.take().unwrap();
            if h.kind == HarmonicType::Tapped {
                h.fret = Some(note.value.to_i8().unwrap() + 12);
            }
            note.effect.harmonic = Some(h);
        }
    }
    Ok(())
}

/// Read note. The first byte is note flags
pub fn read_note_v5(
    song: &mut Song,
    data: &[u8],
    seek: &mut usize,
    note: &mut Note,
    guitar_string: (i8, i8),
    track_index: usize,
) -> GpResult<()> {
    let flags = read_byte(data, seek)?;
    //println!("read_note_v5(), flags: {}", flags);
    note.string = guitar_string.0;
    note.effect.heavy_accentuated_note = (flags & 0x02) == 0x02;
    note.effect.ghost_note = (flags & 0x04) == 0x04;
    note.effect.accentuated_note = (flags & 0x40) == 0x40;
    if (flags & 0x20) == 0x20 {
        note.kind = get_note_type(read_byte(data, seek)?);
    }
    if (flags & 0x10) == 0x10 {
        let v = read_signed_byte(data, seek)?;
        //println!("read_note(), v: {}", v);
        note.velocity = unpack_velocity(v.to_i16().unwrap());
        //println!("read_note(), velocity: {}", note.velocity);
    }
    if (flags & 0x20) == 0x20 {
        let fret = read_signed_byte(data, seek)?;
        let value = if note.kind == NoteType::Tie {
            get_tied_note_value(song, guitar_string.0, track_index)
        } else {
            fret.to_i16().unwrap()
        };
        note.value = value.clamp(0, 99);
        //println!("read_note(), value: {}", note.value);
    }
    if (flags & 0x80) == 0x80 {
        note.effect.left_hand_finger = get_fingering(read_signed_byte(data, seek)?);
        note.effect.right_hand_finger = get_fingering(read_signed_byte(data, seek)?);
    }
    if (flags & 0x01) == 0x01 {
        note.duration_percent = read_double(data, seek)?.to_f32().unwrap();
    }
    note.swap_accidentals = (read_byte(data, seek)? & 0x02) == 0x02;
    if (flags & 0x08) == 0x08 {
        read_note_effects_v4(song, data, seek, note)?;
    }
    Ok(())
}

/// Get note value of tied note
pub fn get_tied_note_value(song: &Song, string_index: i8, track_index: usize) -> i16 {
    //println!("get_tied_note_value()");
    for m in (0usize..song.tracks[track_index].measures.len()).rev() {
        for v in (0usize..song.tracks[track_index].measures[m].voices.len()).rev() {
            for b in 0..song.tracks[track_index].measures[m].voices[v].beats.len() {
                if song.tracks[track_index].measures[m].voices[v].beats[b].status
                    != BeatStatus::Empty
                {
                    for n in 0..song.tracks[track_index].measures[m].voices[v].beats[b]
                        .notes
                        .len()
                    {
                        if song.tracks[track_index].measures[m].voices[v].beats[b].notes[n].string
                            == string_index
                        {
                            return song.tracks[track_index].measures[m].voices[v].beats[b].notes
                                [n]
                                .value;
                        }
                    }
                }
            }
        }
    }
    -1
}
