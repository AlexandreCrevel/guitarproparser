use fraction::ToPrimitive;

use crate::error::GpResult;
use crate::io::primitive::*;
use crate::model::rse::SongRseOps;
use crate::model::song::Song;
use crate::types::mix_table::{MixTableChange, MixTableItem, WahEffect};

pub fn read_mix_table_change_values(
    song: &mut Song,
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
    if song.version.number.0 == 5 {
        mtc.rse = song.read_rse_instrument(data, seek)?;
    }
    if song.version.number == (5, 0, 0) {
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
    if song.version.number >= (5, 0, 0) {
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

pub fn read_mix_table_change_durations(
    song: &Song,
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
        if song.version.number >= (5, 0, 0) {
            mtc.hide_tempo = read_bool(data, seek)?;
        }
    }
    Ok(())
}

pub fn read_mix_table_change_flags(
    song: &Song,
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
    if song.version.number >= (5, 0, 0) {
        mtc.use_rse = (flags & 0x40) == 0x40;
    }
    Ok(flags)
}

pub fn read_wah_effect(
    _song: &Song,
    data: &[u8],
    seek: &mut usize,
    flags: i8,
) -> GpResult<WahEffect> {
    Ok(WahEffect {
        value: read_signed_byte(data, seek)?,
        display: (flags & -0x80) == -0x80,
    })
}
