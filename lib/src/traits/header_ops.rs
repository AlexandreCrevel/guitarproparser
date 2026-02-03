// Header operations trait for reading/writing Guitar Pro measure headers
use std::collections::HashMap;

use crate::error::GpResult;
use crate::types::enums::DirectionSign;
use crate::types::measure::{MeasureHeader, Marker};
use crate::types::song::Clipboard;

pub trait SongHeaderOps {
    fn _add_measure_header(&mut self, header: MeasureHeader);
    fn read_clipboard(&mut self, data: &[u8], seek: &mut usize) -> GpResult<Option<Clipboard>>;
    fn read_measure_headers(
        &mut self,
        data: &[u8],
        seek: &mut usize,
        measure_count: usize,
    ) -> GpResult<()>;
    fn read_measure_headers_v5(
        &mut self,
        data: &[u8],
        seek: &mut usize,
        measure_count: usize,
        directions: &(HashMap<DirectionSign, i16>, HashMap<DirectionSign, i16>),
    ) -> GpResult<()>;
    fn read_measure_header(
        &mut self,
        data: &[u8],
        seek: &mut usize,
        number: usize,
        previous: Option<MeasureHeader>,
    ) -> GpResult<(MeasureHeader, u8)>;
    fn read_measure_header_v5(
        &mut self,
        data: &[u8],
        seek: &mut usize,
        number: usize,
        previous: Option<MeasureHeader>,
    ) -> GpResult<(MeasureHeader, u8)>;
    fn read_repeat_alternative(&mut self, data: &[u8], seek: &mut usize) -> GpResult<u8>;
    fn read_repeat_alternative_v5(&mut self, data: &[u8], seek: &mut usize) -> GpResult<u8>;
    fn read_directions(
        &self,
        data: &[u8],
        seek: &mut usize,
    ) -> GpResult<(HashMap<DirectionSign, i16>, HashMap<DirectionSign, i16>)>;
    fn write_measure_headers(&self, data: &mut Vec<u8>, version: &(u8, u8, u8));
    fn write_measure_header(
        &self,
        data: &mut Vec<u8>,
        header: usize,
        previous: Option<usize>,
        version: &(u8, u8, u8),
    );
    fn write_clipboard(&self, data: &mut Vec<u8>, version: &(u8, u8, u8));
    fn write_directions(&self, data: &mut Vec<u8>);
}

// Helper function for reading markers
pub(crate) fn read_marker(data: &[u8], seek: &mut usize) -> GpResult<Marker> {
    use crate::io::primitive::*;
    let mut marker = Marker {
        title: read_int_size_string(data, seek)?,
        ..Default::default()
    };
    marker.color = read_color(data, seek)?;
    Ok(marker)
}
