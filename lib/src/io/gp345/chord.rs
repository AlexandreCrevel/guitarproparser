// GP3/4/5 Chord I/O implementation
use fraction::ToPrimitive;

use crate::error::GpResult;
use crate::io::primitive::*;
use crate::model::song::Song;
use crate::types::chord::{Chord, Barre, PitchClass};
use crate::types::enums::chord::*;
use crate::types::enums::note::*;
use crate::traits::chord_ops::SongChordOps;

impl SongChordOps for Song {
    /// Read chord diagram. First byte is chord header. If it's set to 0, then following chord is written in
    /// default (GP3) format. If chord header is set to 1, then chord diagram in encoded in more advanced (GP4) format.
    fn read_chord(&self, data: &[u8], seek: &mut usize, string_count: u8) -> GpResult<Chord> {
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
            if self.version.number.0 == 3 {
                self.read_new_format_chord_v3(data, seek, &mut c)?;
            } else {
                self.read_new_format_chord_v4(data, seek, &mut c)?;
            }
        } else {
            if self.version.number.0 == 3 {
                read_byte(data, seek)?;
            }
            self.read_old_format_chord(data, seek, &mut c)?;
        }
        Ok(c)
    }

    /// Read chord diagram encoded in GP3 format. Chord diagram is read as follows:
    /// - Name: `int-byte-size-string`. Name of the chord, e.g. *Em*.
    /// - First fret: `int`. The fret from which the chord is displayed in chord editor.
    /// - List of frets: 6 `ints`. Frets are listed in order: fret on the string 1, fret on the string 2, ..., fret on the
    ///   string 6. If string is untouched then the values of fret is *-1*.
    fn read_old_format_chord(&self, data: &[u8], seek: &mut usize, chord: &mut Chord) -> GpResult<()> {
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

    /// Read new-style (GP4) chord diagram. New-style chord diagram is read as follows:
    /// - Sharp: `bool`. If true, display all semitones as sharps, otherwise display as flats.
    /// - Blank space, 3 `Bytes <byte>`.
    /// - Root: `int`. Values are:
    ///   * -1 for customized chords
    ///   *  0: C
    ///   *  1: C#
    ///   * ...
    /// - Type: `int`. Determines the chord type as followed. See `ChordType` for mapping.
    /// - Chord extension: `int`. See `ChordExtension` for mapping.
    /// - Bass note: `int`. Lowest note of chord as in *C/Am*.
    /// - Tonality: `int`. See `ChordAlteration` for mapping.
    /// - Add: `bool`. Determines if an "add" (added note) is present in the chord.
    /// - Name: `byte-size-string`. Max length is 22.
    /// - Fifth alteration: `int`. Maps to `ChordAlteration`.
    /// - Ninth alteration: `int`. Maps to `ChordAlteration`.
    /// - Eleventh alteration: `int`. Maps to `ChordAlteration`.
    /// - List of frets: 6 `Ints <int>`. Fret values are saved as in default format.
    /// - Count of barres: `int`. Maximum count is 2.
    /// - Barre frets: 2 `Ints <int>`.
    /// - Barre start strings: 2 `Ints <int>`.
    /// - Barre end string: 2 `Ints <int>`.
    /// - Omissions: 7 `Bools <bool>`. If the value is true then note is played in chord.
    /// - Blank space, 1 `byte`.
    fn read_new_format_chord_v3(&self, data: &[u8], seek: &mut usize, chord: &mut Chord) -> GpResult<()> {
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
        chord.tonality = Some(get_chord_alteration(read_int(data, seek)?.to_u8().unwrap())?);
        chord.add = Some(read_bool(data, seek)?);
        chord.name = read_byte_size_string(data, seek, 22)?;
        chord.fifth = Some(get_chord_alteration(read_int(data, seek)?.to_u8().unwrap())?);
        chord.ninth = Some(get_chord_alteration(read_int(data, seek)?.to_u8().unwrap())?);
        chord.eleventh = Some(get_chord_alteration(read_int(data, seek)?.to_u8().unwrap())?);
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

    /// Read new-style (GP4) chord diagram. New-style chord diagram is read as follows:
    /// - Sharp: `bool`. If true, display all semitones as sharps, otherwise display as flats.
    /// - Blank space, 3 `Bytes <byte>`.
    /// - Root: `byte`. Values are:
    ///   * -1 for customized chords
    ///   *  0: C
    ///   *  1: C#
    ///   * ...
    /// - Type: `byte`. Determines the chord type as followed. See `ChordType` for mapping.
    /// - Chord extension: `byte`. See `ChordExtension` for mapping.
    /// - Bass note: `int`. Lowest note of chord as in *C/Am*.
    /// - Tonality: `int`. See `ChordAlteration` for mapping.
    /// - Add: `bool`. Determines if an "add" (added note) is present in the chord.
    /// - Name: `byte-size-string`. Max length is 22.
    /// - Fifth tonality: `byte`. Maps to `ChordExtension`.
    /// - Ninth tonality: `byte`. Maps to `ChordExtension`.
    /// - Eleventh tonality: `byte`. Maps to `ChordExtension`.
    /// - List of frets: 6 `Ints <int>`. Fret values are saved as in default format.
    /// - Count of barres: `byte`. Maximum count is 5.
    /// - Barre frets: 5 `Bytes <byte>`.
    /// - Barre start strings: 5 `Bytes <byte>`.
    /// - Barre end string: 5 `Bytes <byte>`.
    /// - Omissions: 7 `Bools <bool>`. If the value is true then note is played in chord.
    /// - Blank space, 1 `byte`.
    /// - Fingering: 7 `SignedBytes <signed-byte>`. For value mapping, see `Fingering`.
    fn read_new_format_chord_v4(&self, data: &[u8], seek: &mut usize, chord: &mut Chord) -> GpResult<()> {
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
        chord.tonality = Some(get_chord_alteration(read_int(data, seek)?.to_u8().unwrap())?);
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
            chord.fingerings.push(get_fingering(read_signed_byte(data, seek)?));
        }
        chord.show = Some(read_bool(data, seek)?);
        Ok(())
    }

    fn write_chord(&self, data: &mut Vec<u8>, beat: &crate::model::beat::Beat) {
        if let Some(c) = &beat.effect.chord {
            write_bool(data, c.new_format == Some(true));
            if c.new_format == Some(true) {
                self.write_new_format_chord(data, c);
            } else {
                self.write_old_format_chord(data, c);
            }
        }
    }

    fn write_new_format_chord(&self, data: &mut Vec<u8>, chord: &Chord) {
        write_bool(data, chord.sharp == Some(true));
        write_placeholder_default(data, 3);
        //root
        if let Some(r) = &chord.root {
            write_i32(data, r.value.to_i32().unwrap());
        } else {
            write_i32(data, 0);
        }
        //chord type
        if let Some(t) = &chord.kind {
            write_i32(data, from_chord_type(t).to_i32().unwrap());
        } else {
            write_i32(data, 0);
        }
        //chord extension
        if let Some(e) = &chord.extension {
            write_i32(data, from_chord_extension(e).to_i32().unwrap());
        } else {
            write_i32(data, 0);
        }
        //bass
        if let Some(b) = &chord.bass {
            write_i32(data, b.value.to_i32().unwrap());
        } else {
            write_i32(data, 0);
        }
        //tonality
        if let Some(t) = &chord.tonality {
            write_i32(data, from_chord_alteration(t).to_i32().unwrap());
        } else {
            write_i32(data, 0);
        }
        //
        write_bool(data, chord.add == Some(true));
        write_byte_size_string(data, &chord.name);
        write_placeholder_default(data, 22 - chord.name.len());
        //fifth, ninth, eleventh
        if let Some(f) = &chord.fifth {
            write_i32(data, from_chord_alteration(f).to_i32().unwrap());
        } else {
            write_i32(data, 0);
        }
        if let Some(n) = &chord.ninth {
            write_i32(data, from_chord_alteration(n).to_i32().unwrap());
        } else {
            write_i32(data, 0);
        }
        if let Some(e) = &chord.eleventh {
            write_i32(data, from_chord_alteration(e).to_i32().unwrap());
        } else {
            write_i32(data, 0);
        }
        //first fret
        if let Some(ff) = chord.first_fret {
            write_i32(data, ff.to_i32().unwrap());
        } else {
            write_i32(data, 0);
        }
        //strings
        for i in 0..6 {
            if i < chord.strings.len() {
                write_i32(data, chord.strings[i].to_i32().unwrap());
            } else {
                write_i32(data, -1);
            }
        }
        //barre
        let mut barres: Vec<Barre> = Vec::with_capacity(2);
        for i in 0..2usize {
            if i < chord.barres.len() {
                barres.push(chord.barres[i].clone());
            } else {
                break;
            }
        }
        write_i32(data, barres.len().to_i32().unwrap());
        while barres.len() < 2 {
            barres.push(Barre {
                fret: 0,
                start: 0,
                end: 0,
            });
        }
        for b in barres.iter().take(2) {
            write_i32(data, b.fret.to_i32().unwrap());
        }
        for b in barres.iter().take(2) {
            write_i32(data, b.start.to_i32().unwrap());
        }
        for b in barres.iter().take(2) {
            write_i32(data, b.end.to_i32().unwrap());
        }
        //omissions
        for i in 0..7usize {
            if i < chord.omissions.len() {
                write_bool(data, chord.omissions[i]);
            } else {
                write_bool(data, true);
            }
        }
        write_placeholder_default(data, 1);
    }

    fn write_old_format_chord(&self, data: &mut Vec<u8>, chord: &Chord) {
        write_int_byte_size_string(data, &chord.name);
        if let Some(ff) = chord.first_fret {
            write_i32(data, ff.to_i32().unwrap());
        } else {
            write_i32(data, 0);
        }
        for i in 0..6 {
            if i < chord.strings.len() {
                write_i32(data, chord.strings[i].to_i32().unwrap());
            } else {
                write_i32(data, -1);
            }
        }
    }

    fn write_chord_v4(&self, data: &mut Vec<u8>, beat: &crate::model::beat::Beat) {
        if let Some(c) = &beat.effect.chord {
            write_signed_byte(data, 1); //signify GP4 chord format
            write_bool(data, c.sharp == Some(true));
            write_placeholder_default(data, 3);
            //root
            if let Some(r) = &c.root {
                write_i32(data, r.value.to_i32().unwrap());
            } else {
                write_i32(data, 0);
            }
            //chord type
            if let Some(t) = &c.kind {
                write_i32(data, from_chord_type(t).to_i32().unwrap());
            } else {
                write_i32(data, 0);
            }
            //chord extension
            if let Some(e) = &c.extension {
                write_i32(data, from_chord_extension(e).to_i32().unwrap());
            } else {
                write_i32(data, 0);
            }
            //bass
            if let Some(b) = &c.bass {
                write_i32(data, b.value.to_i32().unwrap());
            } else {
                write_i32(data, 0);
            }
            //tonality
            if let Some(t) = &c.tonality {
                write_i32(data, from_chord_alteration(t).to_i32().unwrap());
            } else {
                write_i32(data, 0);
            }
            //
            write_bool(data, c.add == Some(true));
            write_byte_size_string(data, &c.name);
            write_placeholder_default(data, 22 - c.name.len());
            //fifth, ninth, eleventh
            if let Some(f) = &c.fifth {
                write_i32(data, from_chord_alteration(f).to_i32().unwrap());
            } else {
                write_i32(data, 0);
            }
            if let Some(n) = &c.ninth {
                write_i32(data, from_chord_alteration(n).to_i32().unwrap());
            } else {
                write_i32(data, 0);
            }
            if let Some(e) = &c.eleventh {
                write_i32(data, from_chord_alteration(e).to_i32().unwrap());
            } else {
                write_i32(data, 0);
            }
            //first fret
            if let Some(ff) = c.first_fret {
                write_i32(data, ff.to_i32().unwrap());
            } else {
                write_i32(data, 0);
            }
            //strings
            for i in 0..6 {
                if i < c.strings.len() {
                    write_i32(data, c.strings[i].to_i32().unwrap());
                } else {
                    write_i32(data, -1);
                }
            }
            //barre
            let mut barres: Vec<Barre> = Vec::with_capacity(2);
            for i in 0..2usize {
                if i < c.barres.len() {
                    barres.push(c.barres[i].clone());
                } else {
                    break;
                }
            }
            write_i32(data, barres.len().to_i32().unwrap());
            while barres.len() < 5 {
                barres.push(Barre {
                    fret: 0,
                    start: 0,
                    end: 0,
                });
            }
            for b in barres.iter().take(5) {
                write_i32(data, b.fret.to_i32().unwrap());
            }
            for b in barres.iter().take(5) {
                write_i32(data, b.start.to_i32().unwrap());
            }
            for b in barres.iter().take(5) {
                write_i32(data, b.end.to_i32().unwrap());
            }
            //omissions
            for i in 0..7usize {
                if i < c.omissions.len() {
                    write_bool(data, c.omissions[i]);
                } else {
                    write_bool(data, true);
                }
            }
            write_placeholder_default(data, 1);
            for i in 0..7 {
                if i < c.fingerings.len() {
                    write_signed_byte(data, from_fingering(&c.fingerings[i]));
                } else {
                    write_signed_byte(data, -2);
                }
            }
            write_bool(data, c.show == Some(true));
        }
    }
}
