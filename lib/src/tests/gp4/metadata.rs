use crate::tests::read_file;
use crate::model::song::Song;

#[test]
fn test_gp4_copyright() {
    let mut song = Song::default();
    song.read_gp4(&read_file("test/copyright.gp4".into())).unwrap();
}

#[test]
fn test_gp4_capo_fret() {
    let mut song = Song::default();
    song.read_gp4(&read_file("test/capo-fret.gp4".into())).unwrap();
}
