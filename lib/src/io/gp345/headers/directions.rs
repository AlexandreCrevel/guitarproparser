use fraction::ToPrimitive;
use std::collections::HashMap;

use crate::error::GpResult;
use crate::io::primitive::*;
use crate::model::song::Song;
use crate::types::enums::DirectionSign;

pub fn read_directions(
    data: &[u8],
    seek: &mut usize,
) -> GpResult<(HashMap<DirectionSign, i16>, HashMap<DirectionSign, i16>)> {
    let mut signs: HashMap<DirectionSign, i16> = HashMap::with_capacity(4);
    let mut from_signs: HashMap<DirectionSign, i16> = HashMap::with_capacity(15);
    //signs
    signs.insert(DirectionSign::Coda, read_short(data, seek)?);
    signs.insert(DirectionSign::DoubleCoda, read_short(data, seek)?);
    signs.insert(DirectionSign::Segno, read_short(data, seek)?);
    signs.insert(DirectionSign::SegnoSegno, read_short(data, seek)?);
    signs.insert(DirectionSign::Fine, read_short(data, seek)?);
    //from signs
    from_signs.insert(DirectionSign::DaCapo, read_short(data, seek)?);
    from_signs.insert(DirectionSign::DaCapoAlCoda, read_short(data, seek)?);
    from_signs.insert(DirectionSign::DaCapoAlDoubleCoda, read_short(data, seek)?);
    from_signs.insert(DirectionSign::DaCapoAlFine, read_short(data, seek)?);
    from_signs.insert(DirectionSign::DaSegno, read_short(data, seek)?);
    from_signs.insert(DirectionSign::DaSegnoAlCoda, read_short(data, seek)?);
    from_signs.insert(DirectionSign::DaSegnoAlDoubleCoda, read_short(data, seek)?);
    from_signs.insert(DirectionSign::DaSegnoAlFine, read_short(data, seek)?);
    from_signs.insert(DirectionSign::DaSegnoSegno, read_short(data, seek)?);
    from_signs.insert(DirectionSign::DaSegnoSegnoAlCoda, read_short(data, seek)?);
    from_signs.insert(
        DirectionSign::DaSegnoSegnoAlDoubleCoda,
        read_short(data, seek)?,
    );
    from_signs.insert(DirectionSign::DaSegnoSegnoAlFine, read_short(data, seek)?);
    from_signs.insert(DirectionSign::DaCoda, read_short(data, seek)?);
    from_signs.insert(DirectionSign::DaDoubleCoda, read_short(data, seek)?);
    Ok((signs, from_signs))
}

pub fn write_directions(song: &Song, data: &mut Vec<u8>) {
    let mut map: HashMap<DirectionSign, i16> = HashMap::with_capacity(19);
    for i in 1..song.measure_headers.len() {
        if let Some(d) = &song.measure_headers[i].direction {
            map.insert(d.clone(), i.to_i16().unwrap());
        }
    }
    let order: Vec<DirectionSign> = vec![
        DirectionSign::Coda,
        DirectionSign::DoubleCoda,
        DirectionSign::Segno,
        DirectionSign::SegnoSegno,
        DirectionSign::Fine,
        DirectionSign::DaCapo,
        DirectionSign::DaCapoAlCoda,
        DirectionSign::DaCapoAlDoubleCoda,
        DirectionSign::DaCapoAlFine,
        DirectionSign::DaSegno,
        DirectionSign::DaSegnoAlCoda,
        DirectionSign::DaSegnoAlDoubleCoda,
        DirectionSign::DaSegnoAlFine,
        DirectionSign::DaSegnoSegno,
        DirectionSign::DaSegnoSegnoAlCoda,
        DirectionSign::DaSegnoSegnoAlDoubleCoda,
        DirectionSign::DaSegnoSegnoAlFine,
        DirectionSign::DaCoda,
        DirectionSign::DaDoubleCoda,
    ];
    for d in order {
        let x = map.get(&d);
        if let Some(dir) = x {
            write_i16(data, *dir);
        } else {
            write_i16(data, -1);
        }
    }
}
