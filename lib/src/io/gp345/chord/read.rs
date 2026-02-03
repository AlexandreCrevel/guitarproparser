use fraction::ToPrimitive;

use crate::error::GpResult;
use crate::io::primitive::*;
use crate::model::song::Song;
use crate::types::chord::{Barre, Chord, PitchClass};
use crate::types::enums::chord::*;
use crate::types::enums::note::*;

pub fn read_chord(song: &Song, data: &[u8], seek: &mut usize, string_count: u8) -> GpResult<Chord> {
    let mut c = Chord {
        length: string_count,
        strings: vec![-1; string_count.into()],
        ..Default::default()
    };
    for _ in 0..string_count {
        c.strings.push(-1);
    }
    c.new_format = Some(read_bool(data, seek)?);
    if c.new_format == Some(true) {
        if song.version.number.0 == 3 {
            read_new_format_chord_v3(song, data, seek, &mut c)?;
        } else {
            read_new_format_chord_v4(song, data, seek, &mut c)?;
        }
    } else {
        if song.version.number.0 == 3 {
            read_byte(data, seek)?;
        }
        read_old_format_chord(song, data, seek, &mut c)?;
    }
    Ok(c)
}

pub fn read_old_format_chord(
    _song: &Song,
    data: &[u8],
    seek: &mut usize,
    chord: &mut Chord,
) -> GpResult<()> {
    chord.name = read_int_byte_size_string(data, seek)?;
    chord.first_fret = Some(read_int(data, seek)? as u8);
    if chord.first_fret.is_some() {
        for i in 0u8..6u8 {
            let fret = read_int(data, seek)? as i8;
            if i < chord.strings.len().to_u8().unwrap() {
                chord.strings.push(fret);
            }
        }
    }
    Ok(())
}

pub fn read_new_format_chord_v3(
    _song: &Song,
    data: &[u8],
    seek: &mut usize,
    chord: &mut Chord,
) -> GpResult<()> {
    chord.sharp = Some(read_bool(data, seek)?);
    *seek += 3;
    chord.root = Some(PitchClass::from(
        read_int(data, seek)?.to_i8().unwrap(),
        None,
        chord.sharp,
    ));
    chord.kind = Some(get_chord_type(read_int(data, seek)?.to_u8().unwrap()));
    chord.extension = Some(get_chord_extension(read_int(data, seek)?.to_u8().unwrap()));
    chord.bass = Some(PitchClass::from(
        read_int(data, seek)?.to_i8().unwrap(),
        None,
        chord.sharp,
    ));
    chord.tonality = Some(get_chord_alteration(
        read_int(data, seek)?.to_u8().unwrap(),
    )?);
    chord.add = Some(read_bool(data, seek)?);
    chord.name = read_byte_size_string(data, seek, 22)?;
    chord.fifth = Some(get_chord_alteration(
        read_int(data, seek)?.to_u8().unwrap(),
    )?);
    chord.ninth = Some(get_chord_alteration(
        read_int(data, seek)?.to_u8().unwrap(),
    )?);
    chord.eleventh = Some(get_chord_alteration(
        read_int(data, seek)?.to_u8().unwrap(),
    )?);
    chord.first_fret = Some(read_int(data, seek)?.to_u8().unwrap());
    for i in 0u8..6u8 {
        let fret = read_int(data, seek)?.to_i8().unwrap();
        if i < chord.strings.len().to_u8().unwrap() {
            chord.strings.push(fret);
        }
    }
    //barre
    let barre_count = read_int(data, seek)?.to_usize().unwrap();
    let mut barre_frets: Vec<i32> = Vec::with_capacity(2);
    let mut barre_starts: Vec<i32> = Vec::with_capacity(2);
    let mut barre_ends: Vec<i32> = Vec::with_capacity(2);
    for _ in 0u8..2u8 {
        barre_frets.push(read_int(data, seek)?);
    }
    for _ in 0u8..2u8 {
        barre_starts.push(read_int(data, seek)?);
    }
    for _ in 0u8..2u8 {
        barre_ends.push(read_int(data, seek)?);
    }
    for i in 0..barre_count {
        chord.barres.push(Barre {
            fret: barre_frets[i].to_i8().unwrap(),
            start: barre_starts[i].to_i8().unwrap(),
            end: barre_ends[i].to_i8().unwrap(),
        });
    }

    for _ in 0u8..7u8 {
        chord.omissions.push(read_bool(data, seek)?);
    }
    *seek += 1;
    Ok(())
}

pub fn read_new_format_chord_v4(
    _song: &Song,
    data: &[u8],
    seek: &mut usize,
    chord: &mut Chord,
) -> GpResult<()> {
    chord.sharp = Some(read_bool(data, seek)?);
    *seek += 3;
    chord.root = Some(PitchClass::from(
        read_byte(data, seek)?.to_i8().unwrap(),
        None,
        chord.sharp,
    ));
    chord.kind = Some(get_chord_type(read_byte(data, seek)?));
    chord.extension = Some(get_chord_extension(read_byte(data, seek)?));
    let i = read_int(data, seek)?;
    chord.bass = Some(PitchClass::from(i.to_i8().unwrap(), None, chord.sharp));
    chord.tonality = Some(get_chord_alteration(
        read_int(data, seek)?.to_u8().unwrap(),
    )?);
    chord.add = Some(read_bool(data, seek)?);
    chord.name = read_byte_size_string(data, seek, 22)?;
    chord.fifth = Some(get_chord_alteration(read_byte(data, seek)?)?);
    chord.ninth = Some(get_chord_alteration(read_byte(data, seek)?)?);
    chord.eleventh = Some(get_chord_alteration(read_byte(data, seek)?)?);
    chord.first_fret = Some(read_int(data, seek)?.to_u8().unwrap());
    for i in 0u8..7u8 {
        let fret = read_int(data, seek)?.to_i8().unwrap();
        if i < chord.strings.len().to_u8().unwrap() {
            chord.strings.push(fret);
        }
    }
    //barre
    let barre_count = read_byte(data, seek)?.to_usize().unwrap();
    let mut barre_frets: Vec<u8> = Vec::with_capacity(5);
    let mut barre_starts: Vec<u8> = Vec::with_capacity(5);
    let mut barre_ends: Vec<u8> = Vec::with_capacity(5);
    for _ in 0u8..5u8 {
        barre_frets.push(read_byte(data, seek)?);
    }
    for _ in 0u8..5u8 {
        barre_starts.push(read_byte(data, seek)?);
    }
    for _ in 0u8..5u8 {
        barre_ends.push(read_byte(data, seek)?);
    }
    for i in 0..barre_count {
        chord.barres.push(Barre {
            fret: barre_frets[i].to_i8().unwrap(),
            start: barre_starts[i].to_i8().unwrap(),
            end: barre_ends[i].to_i8().unwrap(),
        });
    }
    for _ in 0u8..7u8 {
        chord.omissions.push(read_bool(data, seek)?);
    }
    *seek += 1;
    for _ in 0u8..7u8 {
        chord
            .fingerings
            .push(get_fingering(read_signed_byte(data, seek)?));
    }
    chord.show = Some(read_bool(data, seek)?);
    Ok(())
}
