// GPX beams_stems_ledger_lines tests
use super::super::{read_gpx};


#[test]
fn test_gpx_beams_stems_ledger_lines() {
    let song = read_gpx("test/beams-stems-ledger-lines.gpx");
    assert!(!song.tracks.is_empty());
}

