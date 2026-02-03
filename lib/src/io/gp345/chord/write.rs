use fraction::ToPrimitive;

use crate::io::primitive::*;
use crate::model::song::Song;
use crate::types::chord::{Barre, Chord};
use crate::types::enums::chord::*;
use crate::types::enums::note::*;

pub fn write_chord(_song: &Song, data: &mut Vec<u8>, beat: &crate::model::beat::Beat) {
    if let Some(c) = &beat.effect.chord {
        write_bool(data, c.new_format == Some(true));
        if c.new_format == Some(true) {
            write_new_format_chord(data, c);
        } else {
            write_old_format_chord(data, c);
        }
    }
}

pub fn write_chord_v4(_song: &Song, data: &mut Vec<u8>, beat: &crate::model::beat::Beat) {
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

pub fn write_new_format_chord(data: &mut Vec<u8>, chord: &Chord) {
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

pub fn write_old_format_chord(data: &mut Vec<u8>, chord: &Chord) {
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
