// Chord operations trait
use crate::error::GpResult;
use crate::types::chord::Chord;

pub trait SongChordOps {
    fn read_chord(&self, data: &[u8], seek: &mut usize, string_count: u8) -> GpResult<Chord>;
    fn read_old_format_chord(&self, data: &[u8], seek: &mut usize, chord: &mut Chord) -> GpResult<()>;
    fn read_new_format_chord_v3(&self, data: &[u8], seek: &mut usize, chord: &mut Chord) -> GpResult<()>;
    fn read_new_format_chord_v4(&self, data: &[u8], seek: &mut usize, chord: &mut Chord) -> GpResult<()>;
    fn write_chord(&self, data: &mut Vec<u8>, beat: &crate::model::beat::Beat);
    fn write_new_format_chord(&self, data: &mut Vec<u8>, chord: &Chord);
    fn write_old_format_chord(&self, data: &mut Vec<u8>, chord: &Chord);
    fn write_chord_v4(&self, data: &mut Vec<u8>, beat: &crate::model::beat::Beat);
}
