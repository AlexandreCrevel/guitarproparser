use crate::error::GpResult;
use crate::model::key_signature::Duration;
use crate::model::song::Song;
use crate::traits::note_ops::SongNoteOps;
use crate::types::beat::Beat;
use crate::types::note::{Note, NoteEffect};

pub mod effects;
pub mod read;
pub mod write;

use self::effects::*;
use self::read::*;
use self::write::*;

impl SongNoteOps for Song {
    fn read_notes(
        &mut self,
        data: &[u8],
        seek: &mut usize,
        track_index: usize,
        beat: &mut Beat,
        duration: &Duration,
        note_effect: NoteEffect,
    ) -> GpResult<()> {
        read_notes(self, data, seek, track_index, beat, duration, note_effect)
    }

    fn read_note(
        &mut self,
        data: &[u8],
        seek: &mut usize,
        note: &mut Note,
        guitar_string: (i8, i8),
        track_index: usize,
    ) -> GpResult<()> {
        read_note(self, data, seek, note, guitar_string, track_index)
    }

    fn read_note_v5(
        &mut self,
        data: &[u8],
        seek: &mut usize,
        note: &mut Note,
        guitar_string: (i8, i8),
        track_index: usize,
    ) -> GpResult<()> {
        read_note_v5(self, data, seek, note, guitar_string, track_index)
    }

    fn read_note_effects_v3(&self, data: &[u8], seek: &mut usize, note: &mut Note) -> GpResult<()> {
        read_note_effects_v3(self, data, seek, note)
    }

    fn read_note_effects_v4(
        &mut self,
        data: &[u8],
        seek: &mut usize,
        note: &mut Note,
    ) -> GpResult<()> {
        read_note_effects_v4(self, data, seek, note)
    }

    fn get_tied_note_value(&self, string_index: i8, track_index: usize) -> i16 {
        get_tied_note_value(self, string_index, track_index)
    }

    fn write_notes(
        &self,
        data: &mut Vec<u8>,
        beat: &Beat,
        strings: &[(i8, i8)],
        version: &(u8, u8, u8),
    ) -> GpResult<()> {
        write_notes(self, data, beat, strings, version)
    }

    fn write_note_v3(&self, data: &mut Vec<u8>, note: &Note) -> GpResult<()> {
        write_note_v3(self, data, note)
    }

    fn write_note_v4(
        &self,
        data: &mut Vec<u8>,
        note: &Note,
        strings: &[(i8, i8)],
        version: &(u8, u8, u8),
    ) -> GpResult<()> {
        write_note_v4(self, data, note, strings, version)
    }

    fn write_note_v5(
        &self,
        data: &mut Vec<u8>,
        note: &Note,
        strings: &[(i8, i8)],
        version: &(u8, u8, u8),
    ) -> GpResult<()> {
        write_note_v5(self, data, note, strings, version)
    }

    fn pack_note_flags(&self, note: &Note, version: &(u8, u8, u8)) -> u8 {
        pack_note_flags(note, version)
    }

    fn write_note_effects_v3(&self, data: &mut Vec<u8>, note: &Note) {
        write_note_effects_v3(self, data, note)
    }

    fn write_note_effects(
        &self,
        data: &mut Vec<u8>,
        note: &Note,
        strings: &[(i8, i8)],
        version: &(u8, u8, u8),
    ) -> GpResult<()> {
        write_note_effects(self, data, note, strings, version)
    }
}
