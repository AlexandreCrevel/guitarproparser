use std::collections::HashMap;

use crate::error::GpResult;
use crate::model::song::Song;
use crate::traits::header_ops::SongHeaderOps;
use crate::types::enums::DirectionSign;
use crate::types::measure::MeasureHeader;
use crate::types::song::Clipboard;

pub mod clipboard;
pub mod directions;
pub mod measure;

impl SongHeaderOps for Song {
    fn _add_measure_header(&mut self, header: MeasureHeader) {
        // if the group is closed only the next upcoming header can reopen the group in case of a repeat alternative, so we remove the current group
        //TODO: if header.repeat_open or self.current_repeat_group.is_closed && header.repeat_alternative <= 0 {self.current_repeat_group = RepeatGroup::default();}
        self.measure_headers.push(header);
    }

    fn read_clipboard(&mut self, data: &[u8], seek: &mut usize) -> GpResult<Option<Clipboard>> {
        clipboard::read_clipboard(self, data, seek)
    }

    fn read_measure_headers(
        &mut self,
        data: &[u8],
        seek: &mut usize,
        measure_count: usize,
    ) -> GpResult<()> {
        measure::read_measure_headers(self, data, seek, measure_count)
    }

    fn read_measure_headers_v5(
        &mut self,
        data: &[u8],
        seek: &mut usize,
        measure_count: usize,
        directions: &(HashMap<DirectionSign, i16>, HashMap<DirectionSign, i16>),
    ) -> GpResult<()> {
        measure::read_measure_headers_v5(self, data, seek, measure_count, directions)
    }

    fn read_directions(
        &self,
        data: &[u8],
        seek: &mut usize,
    ) -> GpResult<(HashMap<DirectionSign, i16>, HashMap<DirectionSign, i16>)> {
        directions::read_directions(data, seek)
    }

    fn read_measure_header(
        &mut self,
        data: &[u8],
        seek: &mut usize,
        number: usize,
        previous: Option<MeasureHeader>,
    ) -> GpResult<(MeasureHeader, u8)> {
        measure::read_measure_header(self, data, seek, number, previous)
    }

    fn read_measure_header_v5(
        &mut self,
        data: &[u8],
        seek: &mut usize,
        number: usize,
        previous: Option<MeasureHeader>,
    ) -> GpResult<(MeasureHeader, u8)> {
        measure::read_measure_header_v5(self, data, seek, number, previous)
    }

    fn read_repeat_alternative(&mut self, data: &[u8], seek: &mut usize) -> GpResult<u8> {
        measure::read_repeat_alternative(self, data, seek)
    }

    fn read_repeat_alternative_v5(&mut self, data: &[u8], seek: &mut usize) -> GpResult<u8> {
        measure::read_repeat_alternative_v5(self, data, seek)
    }

    fn write_measure_header(
        &self,
        data: &mut Vec<u8>,
        header: usize,
        previous: Option<usize>,
        version: &(u8, u8, u8),
    ) {
        measure::write_measure_header(self, data, header, previous, version)
    }

    fn write_measure_headers(&self, data: &mut Vec<u8>, version: &(u8, u8, u8)) {
        measure::write_measure_headers(self, data, version)
    }

    fn write_clipboard(&self, data: &mut Vec<u8>, version: &(u8, u8, u8)) {
        clipboard::write_clipboard(self, data, version)
    }

    fn write_directions(&self, data: &mut Vec<u8>) {
        directions::write_directions(self, data)
    }
}
