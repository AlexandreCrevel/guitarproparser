use fraction::ToPrimitive;
use std::collections::HashMap;

use crate::error::GpResult;
use crate::io::primitive::*;
use crate::model::song::Song;
use crate::traits::header_ops::read_marker;
use crate::types::enums::*;
use crate::types::measure::MeasureHeader;

pub fn read_measure_headers(
    song: &mut Song,
    data: &[u8],
    seek: &mut usize,
    measure_count: usize,
) -> GpResult<()> {
    //println!("read_measure_headers()");
    let mut previous: Option<MeasureHeader> = None;
    for i in 1..measure_count + 1 {
        let r: (MeasureHeader, u8) = read_measure_header(song, data, seek, i, previous)?;
        previous = Some(r.0.clone());
        song.measure_headers.push(r.0); //TODO: use add_measure_header
    }
    Ok(())
}

pub fn read_measure_headers_v5(
    song: &mut Song,
    data: &[u8],
    seek: &mut usize,
    measure_count: usize,
    directions: &(HashMap<DirectionSign, i16>, HashMap<DirectionSign, i16>),
) -> GpResult<()> {
    //println!("read_measure_headers_v5()");
    let mut previous: Option<MeasureHeader> = None;
    for i in 1..measure_count + 1 {
        let r: (MeasureHeader, u8) = read_measure_header_v5(song, data, seek, i, previous)?;
        previous = Some(r.0.clone());
        song.measure_headers.push(r.0); //TODO: use add_measure_header
    }
    for s in &directions.0 {
        if s.1 > &-1 {
            song.measure_headers[s.1.to_usize().unwrap() - 1].direction = Some(s.0.clone());
        }
    }
    for s in &directions.1 {
        if s.1 > &-1 {
            song.measure_headers[s.1.to_usize().unwrap() - 1].direction = Some(s.0.clone());
        }
    }
    Ok(())
}

pub fn read_measure_header(
    song: &mut Song,
    data: &[u8],
    seek: &mut usize,
    number: usize,
    previous: Option<MeasureHeader>,
) -> GpResult<(MeasureHeader, u8)> {
    let flag = read_byte(data, seek)?;
    //println!("read_measure_header(), flags: {} \t N: {} \t Measure header count: {}", flag, number, song.measure_headers.len());
    let mut mh = MeasureHeader {
        number: number.to_u16().unwrap(),
        ..Default::default()
    };
    mh.start = 0;
    mh.triplet_feel = song.triplet_feel.clone(); //TODO: use ref & lifetime
                                                 //we need a previous header for the next 2 flags
                                                 //Numerator of the (key) signature
    if (flag & 0x01) == 0x01 {
        mh.time_signature.numerator = read_signed_byte(data, seek)?;
    } else if number > 1 {
        mh.time_signature.numerator = previous.clone().unwrap().time_signature.numerator;
    }
    //Denominator of the (key) signature
    if (flag & 0x02) == 0x02 {
        mh.time_signature.denominator.value = read_signed_byte(data, seek)?.to_u16().unwrap();
    } else if number > 1 {
        mh.time_signature.denominator = previous.clone().unwrap().time_signature.denominator;
    }

    mh.repeat_open = (flag & 0x04) == 0x04; //Beginning of repeat
    if (flag & 0x08) == 0x08 {
        mh.repeat_close = read_signed_byte(data, seek)?;
    } //End of repeat
    if (flag & 0x10) == 0x10 {
        mh.repeat_alternative = if song.version.number.0 == 5 {
            read_repeat_alternative_v5(song, data, seek)?
        } else {
            read_repeat_alternative(song, data, seek)?
        };
    } //Number of alternate ending
    if (flag & 0x20) == 0x20 {
        mh.marker = Some(read_marker(data, seek)?);
    } //Presence of a marker
    if (flag & 0x40) == 0x40 {
        //Tonality of the measure
        mh.key_signature.key = read_signed_byte(data, seek)?;
        mh.key_signature.is_minor = read_signed_byte(data, seek)? != 0;
    } else if mh.number > 1 {
        mh.key_signature = previous.unwrap().key_signature;
    }
    mh.double_bar = (flag & 0x80) == 0x80; //presence of a double bar
    Ok((mh, flag))
}

pub fn read_measure_header_v5(
    song: &mut Song,
    data: &[u8],
    seek: &mut usize,
    number: usize,
    previous: Option<MeasureHeader>,
) -> GpResult<(MeasureHeader, u8)> {
    if previous.is_some() {
        *seek += 1;
    } //always
    let r = read_measure_header(song, data, seek, number, previous.clone())?;
    let mut mh = r.0;
    let flags = r.1;
    //println!("read_measure_header_v5(), flags: {}", flags);
    if mh.repeat_close > -1 {
        mh.repeat_close -= 1;
    }
    if (flags & 0x03) == 0x03 {
        for i in 0..4 {
            mh.time_signature.beams[i] = read_byte(data, seek)?;
        }
    } else {
        mh.time_signature.beams = previous.unwrap().time_signature.beams;
    };
    if (flags & 0x10) == 0 {
        *seek += 1;
    } //always 0
    mh.triplet_feel = get_triplet_feel(read_byte(data, seek)?.to_i8().unwrap())?;
    //println!("################################### {:?}", mh.triplet_feel);
    Ok((mh, flags))
}

pub fn read_repeat_alternative(song: &mut Song, data: &[u8], seek: &mut usize) -> GpResult<u8> {
    //println!("read_repeat_alternative()");
    let value = read_byte(data, seek)?.to_u16().unwrap();
    let mut existing_alternative = 0u16;
    for i in (0..song.measure_headers.len()).rev() {
        if song.measure_headers[i].repeat_open {
            break;
        }
        existing_alternative |= song.measure_headers[i].repeat_alternative.to_u16().unwrap();
    }
    //println!("read_repeat_alternative(), value:  {}, existing_alternative: {}", value, existing_alternative);
    //println!("read_repeat_alternative(), return: {}", ((1 << value) - 1) ^ existing_alternative);
    Ok((((1 << value) - 1) ^ existing_alternative).to_u8().unwrap())
}

pub fn read_repeat_alternative_v5(_song: &mut Song, data: &[u8], seek: &mut usize) -> GpResult<u8> {
    read_byte(data, seek)
}

pub fn write_measure_headers(song: &Song, data: &mut Vec<u8>, version: &(u8, u8, u8)) {
    let mut previous: Option<usize> = None;
    for i in 0..song.measure_headers.len() {
        //song.current_measure_number = Some(song.tracks[0].measures[i].number);
        write_measure_header(song, data, i, previous, version);
        previous = Some(i);
    }
}

pub fn write_measure_header(
    song: &Song,
    data: &mut Vec<u8>,
    header: usize,
    previous: Option<usize>,
    version: &(u8, u8, u8),
) {
    //pack measure header flags
    let mut flags: u8 = 0x00;
    if let Some(p) = previous {
        if song.measure_headers[header].time_signature.numerator
            != song.measure_headers[p].time_signature.numerator
        {
            flags |= 0x01;
        }
        if song.measure_headers[header]
            .time_signature
            .denominator
            .value
            != song.measure_headers[p].time_signature.denominator.value
        {
            flags |= 0x02;
        }
    } else {
        flags |= 0x01;
        flags |= 0x02;
        if song.measure_headers[header].repeat_open {
            flags |= 0x04;
        }
        if song.measure_headers[header].repeat_close > -1 {
            flags |= 0x08;
        }
        if song.measure_headers[header].repeat_alternative > 0 {
            flags |= 0x10;
        }
        if song.measure_headers[header].marker.is_some() {
            flags |= 0x20;
        }
    }
    if version.0 >= 4 {
        if previous.is_none() {
            flags |= 0x40;
        } else if let Some(p) = previous {
            if song.measure_headers[header].key_signature == song.measure_headers[p].key_signature {
                flags |= 0x40;
            }
        }
        if song.measure_headers[header].double_bar {
            flags |= 0x80;
        }
    }
    if version.0 >= 5 {
        if let Some(p) = previous {
            if song.measure_headers[header].time_signature != song.measure_headers[p].time_signature
            {
                flags |= 0x03;
            }
            write_placeholder_default(data, 1);
        }
    }
    //end pack
    //write measure header values
    write_byte(data, flags);
    if (flags & 0x01) == 0x01 {
        write_signed_byte(data, song.measure_headers[header].time_signature.numerator);
    }
    if (flags & 0x02) == 0x02 {
        write_signed_byte(
            data,
            song.measure_headers[header]
                .time_signature
                .denominator
                .value
                .to_i8()
                .unwrap(),
        );
    }
    if (flags & 0x08) == 0x08 {
        write_signed_byte(
            data,
            if version.0 < 5 {
                song.measure_headers[header].repeat_close
            } else {
                song.measure_headers[header].repeat_close + 1
            },
        );
    }
    if (flags & 0x10) == 0x10 {
        //write repeat alternative
        if version.0 == 5 {
            write_byte(data, song.measure_headers[header].repeat_alternative);
        } else {
            let mut first_one = false;
            let mut ra: u8 = 0;
            for i in 0u8..9 - song.measure_headers[header]
                .repeat_alternative
                .leading_zeros()
                .to_u8()
                .unwrap()
            {
                ra = i;
                if (song.measure_headers[header].repeat_alternative & 1 << i) > 0 {
                    first_one = true;
                } else if first_one {
                    break;
                }
            }
            write_byte(data, ra);
        }
    }
    if (flags & 0x20) == 0x20 {
        //write marker
        if let Some(marker) = &song.measure_headers[header].marker {
            write_int_byte_size_string(data, &marker.title);
            write_color(data, marker.color);
        }
    }
    if version.0 >= 4 {
        write_signed_byte(data, song.measure_headers[header].key_signature.key);
        write_signed_byte(
            data,
            i8::from(song.measure_headers[header].key_signature.is_minor),
        );
    }
    if version.0 >= 5 {
        if (flags & 0x03) == 0x03 {
            for i in 0..song.measure_headers[header].time_signature.beams.len() {
                write_byte(data, song.measure_headers[header].time_signature.beams[i]);
            }
        }
        if (flags & 0x10) == 0x10 {
            write_placeholder_default(data, 1);
        }
        write_byte(
            data,
            from_triplet_feel(&song.measure_headers[header].triplet_feel),
        );
    }
}
