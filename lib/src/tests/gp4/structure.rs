use crate::model::song::Song;
use crate::tests::read_file;

#[test]
fn test_gp4_key() {
    let mut song = Song::default();
    song.read_gp4(&read_file("test/Key.gp4".into())).unwrap();
}

#[test]
fn test_gp4_keysig() {
    let mut song = Song::default();
    song.read_gp4(&read_file("test/keysig.gp4".into())).unwrap();
}

#[test]
fn test_gp4_repeat() {
    let mut song = Song::default();
    song.read_gp4(&read_file("test/Repeat.gp4".into())).unwrap();
}

#[test]
fn test_gp4_volta() {
    let mut song = Song::default();
    song.read_gp4(&read_file("test/volta.gp4".into())).unwrap();
}

#[test]
fn test_gp4_tempo() {
    let mut song = Song::default();
    song.read_gp4(&read_file("test/tempo.gp4".into())).unwrap();
}
