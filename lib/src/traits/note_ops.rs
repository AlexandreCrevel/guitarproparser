// Note operations trait for reading/writing Guitar Pro notes
use crate::error::GpResult;
use crate::model::key_signature::Duration;
use crate::types::beat::Beat;
use crate::types::note::{Note, NoteEffect};

pub trait SongNoteOps {
    fn read_notes(
        &mut self,
        data: &[u8],
        seek: &mut usize,
        track_index: usize,
        beat: &mut Beat,
        duration: &Duration,
        note_effect: NoteEffect,
    ) -> GpResult<()>;

    fn read_note(
        &mut self,
        data: &[u8],
        seek: &mut usize,
        note: &mut Note,
        guitar_string: (i8, i8),
        track_index: usize,
    ) -> GpResult<()>;

    fn read_note_v5(
        &mut self,
        data: &[u8],
        seek: &mut usize,
        note: &mut Note,
        guitar_string: (i8, i8),
        track_index: usize,
    ) -> GpResult<()>;

    fn read_note_effects_v3(&self, data: &[u8], seek: &mut usize, note: &mut Note) -> GpResult<()>;

    fn read_note_effects_v4(
        &mut self,
        data: &[u8],
        seek: &mut usize,
        note: &mut Note,
    ) -> GpResult<()>;

    fn get_tied_note_value(&self, string_index: i8, track_index: usize) -> i16;

    fn write_notes(
        &self,
        data: &mut Vec<u8>,
        beat: &Beat,
        strings: &[(i8, i8)],
        version: &(u8, u8, u8),
    ) -> GpResult<()>;

    fn write_note_v3(&self, data: &mut Vec<u8>, note: &Note) -> GpResult<()>;

    fn write_note_v4(
        &self,
        data: &mut Vec<u8>,
        note: &Note,
        strings: &[(i8, i8)],
        version: &(u8, u8, u8),
    ) -> GpResult<()>;

    fn write_note_v5(
        &self,
        data: &mut Vec<u8>,
        note: &Note,
        strings: &[(i8, i8)],
        version: &(u8, u8, u8),
    ) -> GpResult<()>;

    fn pack_note_flags(&self, note: &Note, version: &(u8, u8, u8)) -> u8;

    fn write_note_effects_v3(&self, data: &mut Vec<u8>, note: &Note);

    fn write_note_effects(
        &self,
        data: &mut Vec<u8>,
        note: &Note,
        strings: &[(i8, i8)],
        version: &(u8, u8, u8),
    ) -> GpResult<()>;
}
