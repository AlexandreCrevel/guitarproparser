use fraction::ToPrimitive;

use crate::io::primitive::*;
use crate::model::rse::SongRseOps;
use crate::model::song::Song;
use crate::types::mix_table::MixTableChange;

pub fn write_mix_table_change_values(
    song: &Song,
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
        song.write_rse_instrument(data, &mix_table_change.rse, version);
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

pub fn write_mix_table_change_durations(
    _song: &Song,
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

pub fn write_mix_table_change_flags_v4(
    _song: &Song,
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

pub fn write_mix_table_change_flags_v5(
    _song: &Song,
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
