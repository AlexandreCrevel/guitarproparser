// GP7 beams_stems_ledger_lines tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gp7, read_file};


#[test]
fn test_gp7_beams_stems_ledger_lines() {
    let song = read_gp7("test/beams-stems-ledger-lines.gp");
    assert!(!song.tracks.is_empty());
}

