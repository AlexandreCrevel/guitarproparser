use fraction::ToPrimitive;

use crate::error::GpResult;
use crate::io::primitive::*;
use crate::model::song::Song;
use crate::types::song::Clipboard;

pub fn read_clipboard(
    song: &mut Song,
    data: &[u8],
    seek: &mut usize,
) -> GpResult<Option<Clipboard>> {
    if !song.version.clipboard {
        return Ok(None);
    }
    let mut c = Clipboard {
        start_measure: read_int(data, seek)?,
        ..Default::default()
    };
    c.stop_measure = read_int(data, seek)?;
    c.start_track = read_int(data, seek)?;
    c.stop_track = read_int(data, seek)?;
    if song.version.number.0 == 5 {
        c.start_beat = read_int(data, seek)?;
        c.stop_beat = read_int(data, seek)?;
        c.sub_bar_copy = read_int(data, seek)? != 0;
    }
    println!("read_clipboard(): {:?}", c);
    Ok(Some(c))
}

pub fn write_clipboard(song: &Song, data: &mut Vec<u8>, version: &(u8, u8, u8)) {
    if let Some(c) = &song.clipboard {
        write_i32(data, c.start_measure.to_i32().unwrap());
        write_i32(data, c.stop_measure.to_i32().unwrap());
        write_i32(data, c.start_track.to_i32().unwrap());
        write_i32(data, c.stop_track.to_i32().unwrap());
        if version.0 == 5 {
            write_i32(data, c.start_beat.to_i32().unwrap());
            write_i32(data, c.stop_beat.to_i32().unwrap());
            write_i32(data, i32::from(c.sub_bar_copy));
        }
    }
}
