use crate::error::GpResult;
use crate::model::note::NoteEffect;
use crate::model::song::Song;
use crate::traits::beat_ops::SongBeatOps;
use crate::types::beat::{Beat, BeatEffects, BeatStroke, Voice};
use crate::types::effects::bend::BendEffect;

mod effects;
mod read;
mod write;

impl SongBeatOps for Song {
    fn read_beat(
        &mut self,
        data: &[u8],
        seek: &mut usize,
        voice: &mut Voice,
        start: i64,
        track_index: usize,
    ) -> GpResult<i64> {
        read::read_beat(self, data, seek, voice, start, track_index)
    }

    fn read_beat_v5(
        &mut self,
        data: &[u8],
        seek: &mut usize,
        voice: &mut Voice,
        start: &mut i64,
        track_index: usize,
    ) -> GpResult<i64> {
        read::read_beat_v5(self, data, seek, voice, start, track_index)
    }

    fn read_beat_effects_v3(
        &self,
        data: &[u8],
        seek: &mut usize,
        note_effect: &mut NoteEffect,
    ) -> GpResult<BeatEffects> {
        effects::read_beat_effects_v3(self, data, seek, note_effect)
    }

    fn read_beat_effects_v4(&self, data: &[u8], seek: &mut usize) -> GpResult<BeatEffects> {
        effects::read_beat_effects_v4(self, data, seek)
    }

    fn read_beat_stroke(&self, data: &[u8], seek: &mut usize) -> GpResult<BeatStroke> {
        effects::read_beat_stroke(self, data, seek)
    }

    fn stroke_value(&self, value: i8) -> u8 {
        effects::stroke_value(value)
    }

    fn read_tremolo_bar(&self, data: &[u8], seek: &mut usize) -> GpResult<BendEffect> {
        effects::read_tremolo_bar(data, seek)
    }

    fn write_beat(
        &self,
        data: &mut Vec<u8>,
        beat: &Beat,
        strings: &[(i8, i8)],
        version: &(u8, u8, u8),
    ) -> GpResult<()> {
        write::write_beat(self, data, beat, strings, version)
    }

    fn write_beat_v3(&self, data: &mut Vec<u8>, beat: &Beat) -> GpResult<()> {
        write::write_beat_v3(self, data, beat)
    }

    fn write_beat_effect_v3(&self, data: &mut Vec<u8>, beat: &Beat) {
        effects::write_beat_effect_v3(data, beat)
    }

    fn write_beat_effect_v4(&self, data: &mut Vec<u8>, beat: &Beat, version: &(u8, u8, u8)) {
        effects::write_beat_effect_v4(self, data, beat, version)
    }

    fn write_tremolo_bar(&self, data: &mut Vec<u8>, bar: &Option<BendEffect>) {
        effects::write_tremolo_bar(data, bar)
    }

    fn write_beat_stroke(&self, data: &mut Vec<u8>, stroke: &BeatStroke, version: &(u8, u8, u8)) {
        effects::write_beat_stroke(data, stroke, version)
    }

    fn from_stroke_value(value: u8) -> i8 {
        effects::from_stroke_value(value)
    }
}
