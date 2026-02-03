// Effect operations trait for reading/writing Guitar Pro effects
use crate::error::GpResult;
use crate::types::effects::{BendEffect, GraceEffect, HarmonicEffect, TremoloPickingEffect, TrillEffect};
use crate::types::enums::SlideType;

pub trait SongEffectOps {
    fn read_bend_effect(&self, data: &[u8], seek: &mut usize) -> GpResult<Option<BendEffect>>;
    fn read_grace_effect(&self, data: &[u8], seek: &mut usize) -> GpResult<GraceEffect>;
    fn read_grace_effect_v5(&self, data: &[u8], seek: &mut usize) -> GpResult<GraceEffect>;
    fn read_tremolo_picking(&self, data: &[u8], seek: &mut usize)
        -> GpResult<TremoloPickingEffect>;
    fn read_slides_v5(&self, data: &[u8], seek: &mut usize) -> GpResult<Vec<SlideType>>;
    fn read_harmonic(
        &self,
        data: &[u8],
        seek: &mut usize,
        note: &crate::model::note::Note,
    ) -> GpResult<HarmonicEffect>;
    fn read_harmonic_v5(&mut self, data: &[u8], seek: &mut usize) -> GpResult<HarmonicEffect>;
    fn read_trill(&self, data: &[u8], seek: &mut usize) -> GpResult<TrillEffect>;

    // Write methods
    fn write_bend(&self, data: &mut Vec<u8>, bend: &Option<BendEffect>);
    fn write_grace(&self, data: &mut Vec<u8>, grace: &Option<GraceEffect>);
    fn write_grace_v5(&self, data: &mut Vec<u8>, grace: &Option<GraceEffect>);
    fn write_harmonic(
        &self,
        data: &mut Vec<u8>,
        note: &crate::model::note::Note,
        strings: &[(i8, i8)],
    ) -> GpResult<()>;
    fn write_harmonic_v5(
        &self,
        data: &mut Vec<u8>,
        note: &crate::model::note::Note,
        strings: &[(i8, i8)],
    ) -> GpResult<()>;
    fn write_slides_v5(&self, data: &mut Vec<u8>, slides: &[SlideType]);
}
