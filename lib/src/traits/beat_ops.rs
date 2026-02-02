// Beat operations trait for reading/writing Guitar Pro beats
use crate::error::GpResult;
use crate::model::note::NoteEffect;
use crate::types::beat::{Beat, BeatEffects, BeatStroke, Voice};
use crate::types::effects::BendEffect;

pub trait SongBeatOps {
    fn read_beat(
        &mut self,
        data: &[u8],
        seek: &mut usize,
        voice: &mut Voice,
        start: i64,
        track_index: usize,
    ) -> GpResult<i64>;

    fn read_beat_v5(
        &mut self,
        data: &[u8],
        seek: &mut usize,
        voice: &mut Voice,
        start: &mut i64,
        track_index: usize,
    ) -> GpResult<i64>;

    fn read_beat_effects_v3(
        &self,
        data: &[u8],
        seek: &mut usize,
        note_effect: &mut NoteEffect,
    ) -> GpResult<BeatEffects>;

    fn read_beat_effects_v4(&self, data: &[u8], seek: &mut usize) -> GpResult<BeatEffects>;

    fn read_beat_stroke(&self, data: &[u8], seek: &mut usize) -> GpResult<BeatStroke>;

    fn stroke_value(&self, value: i8) -> u8;

    fn read_tremolo_bar(&self, data: &[u8], seek: &mut usize) -> GpResult<BendEffect>;

    fn write_beat_v3(&self, data: &mut Vec<u8>, beat: &Beat) -> GpResult<()>;

    fn write_beat(
        &self,
        data: &mut Vec<u8>,
        beat: &Beat,
        strings: &[(i8, i8)],
        version: &(u8, u8, u8),
    ) -> GpResult<()>;

    fn write_beat_effect_v3(&self, data: &mut Vec<u8>, beat: &Beat);

    fn write_beat_effect_v4(&self, data: &mut Vec<u8>, beat: &Beat, version: &(u8, u8, u8));

    fn write_tremolo_bar(&self, data: &mut Vec<u8>, bar: &Option<BendEffect>);

    fn write_beat_stroke(&self, data: &mut Vec<u8>, stroke: &BeatStroke, version: &(u8, u8, u8));

    fn from_stroke_value(value: u8) -> i8;
}
