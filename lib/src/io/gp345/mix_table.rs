// GP3/4/5 MixTable I/O implementation
use fraction::ToPrimitive;

use crate::error::GpResult;
use crate::io::primitive::*;
use crate::model::song::Song;
use crate::model::rse::SongRseOps;
use crate::types::mix_table::{MixTableItem, MixTableChange, WahEffect, WAH_EFFECT_NONE};
use crate::traits::mix_table_ops::SongMixTableOps;

impl SongMixTableOps for Song {
    /// Read mix table change. List of values is read first. See `read_values()`.
    ///
    /// List of values is followed by the list of durations for parameters that have changed. See `read_durations()`.
    ///
    /// Mix table change in Guitar Pro 4 format extends Guitar Pro 3 format. It constists of `values <read_mix_table_change_values()>`,
    /// `durations <read_mix_table_change_durations()>`, and, new to GP3, `flags <read_mix_table_change_flags()>`.
    ///
    /// Mix table change was modified to support RSE instruments. It is read as in Guitar Pro 3 and is followed by:
    /// - Wah effect. See :meth:`read_wah_effect()`.
    /// - RSE instrument effect. See :meth:`read_rse_instrument_effect()`.
    fn read_mix_table_change(&mut self, data: &[u8], seek: &mut usize) -> GpResult<MixTableChange> {
        let mut tc = MixTableChange::default();
        self.read_mix_table_change_values(data, seek, &mut tc)?;
        self.read_mix_table_change_durations(data, seek, &mut tc)?;
        if self.version.number >= (4, 0, 0) {
            let flags = self.read_mix_table_change_flags(data, seek, &mut tc)?;
            if self.version.number >= (5, 0, 0) {
                tc.wah = Some(self.read_wah_effect(data, seek, flags)?);
                self.read_rse_instrument_effect(data, seek, &mut tc.rse)?;
            }
        }
        Ok(tc)
    }

    /// Read mix table change values. Mix table change values consist of 7 `signed-byte` and an `int`, which correspond to:
    /// - instrument
    /// - RSE instrument. See `read_rse_instrument()` (GP5).
    /// - volume
    /// - balance
    /// - chorus
    /// - reverb
    /// - phaser
    /// - tremolo
    /// - Tempo name: `int-byte-size-string` (GP5).
    /// - tempo
    ///
    /// If signed byte is *-1* then corresponding parameter hasn't changed.
    fn read_mix_table_change_values(
        &mut self,
        data: &[u8],
        seek: &mut usize,
        mtc: &mut MixTableChange,
    ) -> GpResult<()> {
        //instrument
        let b = read_signed_byte(data, seek)?;
        if b >= 0 {
            mtc.instrument = Some(MixTableItem {
                value: b.to_u8().unwrap(),
                ..Default::default()
            });
        }
        //RSE instrument GP5
        if self.version.number.0 == 5 {
            mtc.rse = self.read_rse_instrument(data, seek)?;
        }
        if self.version.number == (5, 0, 0) {
            *seek += 1;
        }
        //volume
        let b = read_signed_byte(data, seek)?;
        if b >= 0 {
            mtc.volume = Some(MixTableItem {
                value: b.to_u8().unwrap(),
                ..Default::default()
            });
        }
        //balance
        let b = read_signed_byte(data, seek)?;
        if b >= 0 {
            mtc.balance = Some(MixTableItem {
                value: b.to_u8().unwrap(),
                ..Default::default()
            });
        }
        //chorus
        let b = read_signed_byte(data, seek)?;
        if b >= 0 {
            mtc.chorus = Some(MixTableItem {
                value: b.to_u8().unwrap(),
                ..Default::default()
            });
        }
        //reverb
        let b = read_signed_byte(data, seek)?;
        if b >= 0 {
            mtc.reverb = Some(MixTableItem {
                value: b.to_u8().unwrap(),
                ..Default::default()
            });
        }
        //phaser
        let b = read_signed_byte(data, seek)?;
        if b >= 0 {
            mtc.phaser = Some(MixTableItem {
                value: b.to_u8().unwrap(),
                ..Default::default()
            });
        }
        //tremolo
        let b = read_signed_byte(data, seek)?;
        if b >= 0 {
            mtc.tremolo = Some(MixTableItem {
                value: b.to_u8().unwrap(),
                ..Default::default()
            });
        }
        //tempo
        if self.version.number >= (5, 0, 0) {
            mtc.tempo_name = read_int_byte_size_string(data, seek)?;
        }
        let b = read_int(data, seek)?;
        if b >= 0 {
            mtc.tempo = Some(MixTableItem {
                value: b.clamp(0, 255) as u8,
                ..Default::default()
            });
        }
        Ok(())
    }

    /// Read mix table change durations. Durations are read for each non-null `MixTableItem`. Durations are encoded in `signed-byte`.
    ///
    /// If tempo did change, then one :ref:`bool` is read. If it's true, then tempo change won't be displayed on the score.
    fn read_mix_table_change_durations(
        &self,
        data: &[u8],
        seek: &mut usize,
        mtc: &mut MixTableChange,
    ) -> GpResult<()> {
        if let Some(ref mut item) = mtc.volume {
            item.duration = read_signed_byte(data, seek)?.to_u8().unwrap_or(0);
        }
        if let Some(ref mut item) = mtc.balance {
            item.duration = read_signed_byte(data, seek)?.to_u8().unwrap_or(0);
        }
        if let Some(ref mut item) = mtc.chorus {
            item.duration = read_signed_byte(data, seek)?.to_u8().unwrap_or(0);
        }
        if let Some(ref mut item) = mtc.reverb {
            item.duration = read_signed_byte(data, seek)?.to_u8().unwrap_or(0);
        }
        if let Some(ref mut item) = mtc.phaser {
            item.duration = read_signed_byte(data, seek)?.to_u8().unwrap_or(0);
        }
        if let Some(ref mut item) = mtc.tremolo {
            item.duration = read_signed_byte(data, seek)?.to_u8().unwrap_or(0);
        }
        if let Some(ref mut item) = mtc.tempo {
            item.duration = read_signed_byte(data, seek)?.to_u8().unwrap_or(0);
            mtc.hide_tempo = false;
            if self.version.number >= (5, 0, 0) {
                mtc.hide_tempo = read_bool(data, seek)?;
            }
        }
        Ok(())
    }

    /// Read mix table change flags (Guitar Pro 4). The meaning of flags:
    /// - *0x01*: change volume for all tracks
    /// - *0x02*: change balance for all tracks
    /// - *0x04*: change chorus for all tracks
    /// - *0x08*: change reverb for all tracks
    /// - *0x10*: change phaser for all tracks
    /// - *0x20*: change tremolo for all tracks
    ///
    /// In GP5, there is one additional flag:
    /// - *0x40*: use RSE
    /// - *0x80*: show wah-wah
    fn read_mix_table_change_flags(
        &self,
        data: &[u8],
        seek: &mut usize,
        mtc: &mut MixTableChange,
    ) -> GpResult<i8> {
        let flags = read_signed_byte(data, seek)?;
        if mtc.volume.is_some() {
            let mut e = mtc.volume.take().unwrap();
            e.all_tracks = (flags & 0x01) == 0x01;
            mtc.volume = Some(e);
        }
        if mtc.balance.is_some() {
            let mut e = mtc.balance.take().unwrap();
            e.all_tracks = (flags & 0x01) == 0x01;
            mtc.balance = Some(e);
        }
        if mtc.chorus.is_some() {
            let mut e = mtc.chorus.take().unwrap();
            e.all_tracks = (flags & 0x01) == 0x01;
            mtc.chorus = Some(e);
        }
        if mtc.reverb.is_some() {
            let mut e = mtc.reverb.take().unwrap();
            e.all_tracks = (flags & 0x01) == 0x01;
            mtc.reverb = Some(e);
        }
        if mtc.phaser.is_some() {
            let mut e = mtc.phaser.take().unwrap();
            e.all_tracks = (flags & 0x01) == 0x01;
            mtc.phaser = Some(e);
        }
        if mtc.tremolo.is_some() {
            let mut e = mtc.tremolo.take().unwrap();
            e.all_tracks = (flags & 0x01) == 0x01;
            mtc.tremolo = Some(e);
        }
        if self.version.number >= (5, 0, 0) {
            mtc.use_rse = (flags & 0x40) == 0x40;
        }
        Ok(flags)
    }

    /// Read wah-wah.
    /// - Wah value: :ref:`signed-byte`. See `WahEffect` for value mapping.
    fn read_wah_effect(&self, data: &[u8], seek: &mut usize, flags: i8) -> GpResult<WahEffect> {
        Ok(WahEffect {
            value: read_signed_byte(data, seek)?,
            display: (flags & -0x80) == -0x80,
        })
    }

    fn write_mix_table_change(
        &self,
        data: &mut Vec<u8>,
        mix_table_change: &Option<MixTableChange>,
        version: &(u8, u8, u8),
    ) {
        if let Some(mtc) = mix_table_change {
            self.write_mix_table_change_values(data, mtc, version);
            self.write_mix_table_change_durations(data, mtc, version);
            if version.0 == 4 {
                self.write_mix_table_change_flags_v4(data, mtc);
            }
            if version.0 == 5 {
                self.write_mix_table_change_flags_v5(data, mtc);
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
        //instrument
        if let Some(i) = &mix_table_change.instrument {
            write_signed_byte(data, i.value.to_i8().unwrap());
        } else {
            write_signed_byte(data, -1);
        }
        if version.0 >= 5 {
            self.write_rse_instrument(data, &mix_table_change.rse, version);
        }
        if version == &(5, 0, 0) {
            write_placeholder_default(data, 1);
        }
        //volume
        if let Some(i) = &mix_table_change.volume {
            write_signed_byte(data, i.value.to_i8().unwrap());
        } else {
            write_signed_byte(data, -1);
        }
        //balance
        if let Some(i) = &mix_table_change.balance {
            write_signed_byte(data, i.value.to_i8().unwrap());
        } else {
            write_signed_byte(data, -1);
        }
        //chorus
        if let Some(i) = &mix_table_change.chorus {
            write_signed_byte(data, i.value.to_i8().unwrap());
        } else {
            write_signed_byte(data, -1);
        }
        //reverb
        if let Some(i) = &mix_table_change.reverb {
            write_signed_byte(data, i.value.to_i8().unwrap());
        } else {
            write_signed_byte(data, -1);
        }
        //phaser
        if let Some(i) = &mix_table_change.phaser {
            write_signed_byte(data, i.value.to_i8().unwrap());
        } else {
            write_signed_byte(data, -1);
        }
        //tremolo
        if let Some(i) = &mix_table_change.tremolo {
            write_signed_byte(data, i.value.to_i8().unwrap());
        } else {
            write_signed_byte(data, -1);
        }
        //tempo
        if let Some(i) = &mix_table_change.tempo {
            write_signed_byte(data, i.value.to_i8().unwrap());
        } else {
            write_signed_byte(data, -1);
        }
        if version.0 >= 5 {
            write_int_byte_size_string(data, &mix_table_change.tempo_name);
            if let Some(t) = &mix_table_change.tempo {
                write_i32(data, t.value.to_i32().unwrap());
            } else {
                write_i32(data, -1);
            }
        }
    }

    fn write_mix_table_change_durations(
        &self,
        data: &mut Vec<u8>,
        mix_table_change: &MixTableChange,
        version: &(u8, u8, u8),
    ) {
        //volume
        if let Some(i) = &mix_table_change.volume {
            write_signed_byte(data, i.duration.to_i8().unwrap());
        } else {
            write_signed_byte(data, -1);
        }
        //balance
        if let Some(i) = &mix_table_change.balance {
            write_signed_byte(data, i.duration.to_i8().unwrap());
        } else {
            write_signed_byte(data, -1);
        }
        //chorus
        if let Some(i) = &mix_table_change.chorus {
            write_signed_byte(data, i.duration.to_i8().unwrap());
        } else {
            write_signed_byte(data, -1);
        }
        //reverb
        if let Some(i) = &mix_table_change.reverb {
            write_signed_byte(data, i.duration.to_i8().unwrap());
        } else {
            write_signed_byte(data, -1);
        }
        //phaser
        if let Some(i) = &mix_table_change.phaser {
            write_signed_byte(data, i.duration.to_i8().unwrap());
        } else {
            write_signed_byte(data, -1);
        }
        //tremolo
        if let Some(i) = &mix_table_change.tremolo {
            write_signed_byte(data, i.duration.to_i8().unwrap());
        } else {
            write_signed_byte(data, -1);
        }
        //tempo
        if let Some(i) = &mix_table_change.tempo {
            write_signed_byte(data, i.duration.to_i8().unwrap());
            if version > &(5, 0, 0) {
                write_bool(data, mix_table_change.hide_tempo);
            }
        } else {
            write_signed_byte(data, -1);
        }
    }

    fn write_mix_table_change_flags_v4(
        &self,
        data: &mut Vec<u8>,
        mix_table_change: &MixTableChange,
    ) {
        let mut flags = 0i8;
        if let Some(i) = &mix_table_change.volume {
            if i.all_tracks {
                flags |= 0x01;
            }
        }
        if let Some(i) = &mix_table_change.balance {
            if i.all_tracks {
                flags |= 0x02;
            }
        }
        if let Some(i) = &mix_table_change.chorus {
            if i.all_tracks {
                flags |= 0x04;
            }
        }
        if let Some(i) = &mix_table_change.reverb {
            if i.all_tracks {
                flags |= 0x08;
            }
        }
        if let Some(i) = &mix_table_change.phaser {
            if i.all_tracks {
                flags |= 0x10;
            }
        }
        if let Some(i) = &mix_table_change.tremolo {
            if i.all_tracks {
                flags |= 0x20;
            }
        }
        write_signed_byte(data, flags);
    }

    fn write_mix_table_change_flags_v5(
        &self,
        data: &mut Vec<u8>,
        mix_table_change: &MixTableChange,
    ) {
        let mut flags = 0u8;
        if let Some(i) = &mix_table_change.volume {
            if i.all_tracks {
                flags |= 0x01;
            }
        }
        if let Some(i) = &mix_table_change.balance {
            if i.all_tracks {
                flags |= 0x02;
            }
        }
        if let Some(i) = &mix_table_change.chorus {
            if i.all_tracks {
                flags |= 0x04;
            }
        }
        if let Some(i) = &mix_table_change.reverb {
            if i.all_tracks {
                flags |= 0x08;
            }
        }
        if let Some(i) = &mix_table_change.phaser {
            if i.all_tracks {
                flags |= 0x10;
            }
        }
        if let Some(i) = &mix_table_change.tremolo {
            if i.all_tracks {
                flags |= 0x20;
            }
        }
        if mix_table_change.use_rse {
            flags |= 0x40;
        }
        if let Some(w) = &mix_table_change.wah {
            if w.display {
                flags |= 0x80;
            }
        }
        write_byte(data, flags);
    }
}
