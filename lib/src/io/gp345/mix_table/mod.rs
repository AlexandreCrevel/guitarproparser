use crate::error::GpResult;
use crate::io::primitive::*;
use crate::model::rse::SongRseOps;
use crate::model::song::Song;
use crate::traits::mix_table_ops::SongMixTableOps;
use crate::types::mix_table::{MixTableChange, WahEffect, WAH_EFFECT_NONE};

pub mod read;
pub mod write;

impl SongMixTableOps for Song {
    fn read_mix_table_change(&mut self, data: &[u8], seek: &mut usize) -> GpResult<MixTableChange> {
        let mut tc = MixTableChange::default();
        read::read_mix_table_change_values(self, data, seek, &mut tc)?;
        read::read_mix_table_change_durations(self, data, seek, &mut tc)?;
        if self.version.number >= (4, 0, 0) {
            let flags = read::read_mix_table_change_flags(self, data, seek, &mut tc)?;
            if self.version.number >= (5, 0, 0) {
                tc.wah = Some(read::read_wah_effect(self, data, seek, flags)?);
                self.read_rse_instrument_effect(data, seek, &mut tc.rse)?;
            }
        }
        Ok(tc)
    }

    fn read_mix_table_change_values(
        &mut self,
        data: &[u8],
        seek: &mut usize,
        mtc: &mut MixTableChange,
    ) -> GpResult<()> {
        read::read_mix_table_change_values(self, data, seek, mtc)
    }

    fn read_mix_table_change_durations(
        &self,
        data: &[u8],
        seek: &mut usize,
        mtc: &mut MixTableChange,
    ) -> GpResult<()> {
        read::read_mix_table_change_durations(self, data, seek, mtc)
    }

    fn read_mix_table_change_flags(
        &self,
        data: &[u8],
        seek: &mut usize,
        mtc: &mut MixTableChange,
    ) -> GpResult<i8> {
        read::read_mix_table_change_flags(self, data, seek, mtc)
    }

    fn read_wah_effect(&self, data: &[u8], seek: &mut usize, flags: i8) -> GpResult<WahEffect> {
        read::read_wah_effect(self, data, seek, flags)
    }

    fn write_mix_table_change(
        &self,
        data: &mut Vec<u8>,
        mix_table_change: &Option<MixTableChange>,
        version: &(u8, u8, u8),
    ) {
        if let Some(mtc) = mix_table_change {
            write::write_mix_table_change_values(self, data, mtc, version);
            write::write_mix_table_change_durations(self, data, mtc, version);
            if version.0 == 4 {
                write::write_mix_table_change_flags_v4(self, data, mtc);
            }
            if version.0 == 5 {
                write::write_mix_table_change_flags_v5(self, data, mtc);
                if let Some(w) = &mtc.wah {
                    write_signed_byte(data, w.value);
                } else {
                    write_signed_byte(data, WAH_EFFECT_NONE);
                }
                self.write_rse_instrument_effect(data, &mtc.rse);
            }
        }
    }

    fn write_mix_table_change_values(
        &self,
        data: &mut Vec<u8>,
        mix_table_change: &MixTableChange,
        version: &(u8, u8, u8),
    ) {
        write::write_mix_table_change_values(self, data, mix_table_change, version)
    }

    fn write_mix_table_change_durations(
        &self,
        data: &mut Vec<u8>,
        mix_table_change: &MixTableChange,
        version: &(u8, u8, u8),
    ) {
        write::write_mix_table_change_durations(self, data, mix_table_change, version)
    }

    fn write_mix_table_change_flags_v4(
        &self,
        data: &mut Vec<u8>,
        mix_table_change: &MixTableChange,
    ) {
        write::write_mix_table_change_flags_v4(self, data, mix_table_change)
    }

    fn write_mix_table_change_flags_v5(
        &self,
        data: &mut Vec<u8>,
        mix_table_change: &MixTableChange,
    ) {
        write::write_mix_table_change_flags_v5(self, data, mix_table_change)
    }
}
