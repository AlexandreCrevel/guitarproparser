use crate::model::song::Song;
use crate::tests::read_file;

#[test]
fn test_gp3_copyright() {
    let mut song = Song::default();
    song.read_gp3(&read_file("test/copyright.gp3".into())).unwrap();
}

#[test]
fn test_gp3_capo_fret() {
    let mut song = Song::default();
    song.read_gp3(&read_file("test/capo-fret.gp3".into())).unwrap();
}

#[test]
fn test_gp3_tempo() {
    let mut song = Song::default();
    song.read_gp3(&read_file("test/tempo.gp3".into())).unwrap();
}
