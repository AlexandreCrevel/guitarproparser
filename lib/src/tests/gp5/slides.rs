// GP5 slides tests
use crate::model::song::Song;
use super::super::read_file;

#[test]
fn test_gp5_slides() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/Slides.gp5")))
        .unwrap();
}

#[test]
fn test_gp5_legato_slide() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/legato-slide.gp5")))
        .unwrap();
}

#[test]
fn test_gp5_shift_slide() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/shift-slide.gp5")))
        .unwrap();
}

#[test]
fn test_gp5_slide_in_above() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/slide-in-above.gp5")))
        .unwrap();
}

#[test]
fn test_gp5_slide_in_below() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/slide-in-below.gp5")))
        .unwrap();
}

#[test]
fn test_gp5_slide_out_down() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/slide-out-down.gp5")))
        .unwrap();
}

#[test]
fn test_gp5_slide_out_up() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/slide-out-up.gp5")))
        .unwrap();
}

