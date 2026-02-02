// MixTable operations trait
use crate::error::GpResult;
use crate::types::mix_table::{MixTableChange, WahEffect};

pub trait SongMixTableOps {
    fn read_mix_table_change(&mut self, data: &[u8], seek: &mut usize) -> GpResult<MixTableChange>;
    fn read_mix_table_change_values(
        &mut self,
        data: &[u8],
        seek: &mut usize,
        mtc: &mut MixTableChange,
    ) -> GpResult<()>;
    fn read_mix_table_change_durations(
        &self,
        data: &[u8],
        seek: &mut usize,
        mtc: &mut MixTableChange,
    ) -> GpResult<()>;
    fn read_mix_table_change_flags(
        &self,
        data: &[u8],
        seek: &mut usize,
        mtc: &mut MixTableChange,
    ) -> GpResult<i8>;
    fn read_wah_effect(&self, data: &[u8], seek: &mut usize, flags: i8) -> GpResult<WahEffect>;
    fn write_mix_table_change(
        &self,
        data: &mut Vec<u8>,
        mix_table_change: &Option<MixTableChange>,
        version: &(u8, u8, u8),
    );
    fn write_mix_table_change_values(
        &self,
        data: &mut Vec<u8>,
        mix_table_change: &MixTableChange,
        version: &(u8, u8, u8),
    );
    fn write_mix_table_change_durations(
        &self,
        data: &mut Vec<u8>,
        mix_table_change: &MixTableChange,
        version: &(u8, u8, u8),
    );
    fn write_mix_table_change_flags_v4(
        &self,
        data: &mut Vec<u8>,
        mix_table_change: &MixTableChange,
    );
    fn write_mix_table_change_flags_v5(
        &self,
        data: &mut Vec<u8>,
        mix_table_change: &MixTableChange,
    );
}
