use crate::error::GpResult;
use crate::model::song::Song;
use crate::traits::chord_ops::SongChordOps;
use crate::types::chord::Chord;

pub mod read;
pub mod write;

impl SongChordOps for Song {
    fn read_chord(&self, data: &[u8], seek: &mut usize, string_count: u8) -> GpResult<Chord> {
        read::read_chord(self, data, seek, string_count)
    }

    fn read_old_format_chord(
        &self,
        data: &[u8],
        seek: &mut usize,
        chord: &mut Chord,
    ) -> GpResult<()> {
        read::read_old_format_chord(self, data, seek, chord)
    }

    fn read_new_format_chord_v3(
        &self,
        data: &[u8],
        seek: &mut usize,
        chord: &mut Chord,
    ) -> GpResult<()> {
        read::read_new_format_chord_v3(self, data, seek, chord)
    }

    fn read_new_format_chord_v4(
        &self,
        data: &[u8],
        seek: &mut usize,
        chord: &mut Chord,
    ) -> GpResult<()> {
        read::read_new_format_chord_v4(self, data, seek, chord)
    }

    fn write_chord(&self, data: &mut Vec<u8>, beat: &crate::model::beat::Beat) {
        write::write_chord(self, data, beat)
    }

    fn write_new_format_chord(&self, data: &mut Vec<u8>, chord: &Chord) {
        write::write_new_format_chord(data, chord)
    }

    fn write_old_format_chord(&self, data: &mut Vec<u8>, chord: &Chord) {
        write::write_old_format_chord(data, chord)
    }

    fn write_chord_v4(&self, data: &mut Vec<u8>, beat: &crate::model::beat::Beat) {
        write::write_chord_v4(self, data, beat)
    }
}
