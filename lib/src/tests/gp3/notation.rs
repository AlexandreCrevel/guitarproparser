use crate::tests::read_file;
use crate::model::song::Song;

#[test]
fn test_gp3_dotted_gliss() {
    let mut song = Song::default();
    song.read_gp3(&read_file("test/dotted-gliss.gp3".into())).unwrap();
}

#[test]
fn test_gp3_high_pitch() {
    let mut song = Song::default();
    song.read_gp3(&read_file("test/high-pitch.gp3".into())).unwrap();
}

#[test]
fn test_gp3_volta() {
    let mut song = Song::default();
    song.read_gp3(&read_file("test/volta.gp3".into())).unwrap();
}
