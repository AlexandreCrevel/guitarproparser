use crate::model::song::Song;
use crate::tests::read_file;

#[test]
fn test_gp4_rest_centered() {
    let mut song = Song::default();
    song.read_gp4(&read_file("test/rest-centered.gp4".into())).unwrap();
}

#[test]
fn test_gp4_test_irr_tuplet() {
    let mut song = Song::default();
    song.read_gp4(&read_file("test/testIrrTuplet.gp4".into())).unwrap();
}

#[test]
fn test_gp4_tuplet_with_slur() {
    let mut song = Song::default();
    song.read_gp4(&read_file("test/tuplet-with-slur.gp4".into())).unwrap();
}
