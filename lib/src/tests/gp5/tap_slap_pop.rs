// GP5 tap_slap_pop tests
use crate::model::song::Song;
use super::super::read_file;

#[test]
fn test_gp5_tap_slap_pop() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/tap-slap-pop.gp5")))
        .unwrap();
}

