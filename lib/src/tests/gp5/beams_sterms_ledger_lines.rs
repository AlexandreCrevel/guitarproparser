// GP5 beams_sterms_ledger_lines tests
use crate::model::song::Song;
use crate::*;
use super::super::read_file;

#[test]
fn test_gp5_beams_sterms_ledger_lines() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from(
        "test/beams-stems-ledger-lines.gp5",
    )))
    .unwrap();
}

