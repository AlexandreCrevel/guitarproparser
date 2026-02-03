use crate::tests::read_file;
use crate::model::song::Song;

#[test]
fn test_gp4_strokes() {
    let mut song = Song::default();
    song.read_gp4(&read_file("test/Strokes.gp4".into())).unwrap();
}

#[test]
fn test_gp4_pick_up_down() {
    let mut song = Song::default();
    song.read_gp4(&read_file("test/pick-up-down.gp4".into())).unwrap();
}

#[test]
fn test_gp4_sforzato() {
    let mut song = Song::default();
    song.read_gp4(&read_file("test/sforzato.gp4".into())).unwrap();
}

#[test]
fn test_gp4_slur() {
    let mut song = Song::default();
    song.read_gp4(&read_file("test/slur.gp4".into())).unwrap();
}

#[test]
fn test_gp4_trill() {
    let mut song = Song::default();
    song.read_gp4(&read_file("test/trill.gp4".into())).unwrap();
}

#[test]
fn test_gp4_fingering() {
    let mut song = Song::default();
    song.read_gp4(&read_file("test/fingering.gp4".into())).unwrap();
}
