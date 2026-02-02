// GPX beams_stems_ledger_lines tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gpx, read_file};


#[test]
fn test_gpx_beams_stems_ledger_lines() {
    let song = read_gpx("test/beams-stems-ledger-lines.gpx");
    assert!(!song.tracks.is_empty());
}

