// GP4 slides tests
use crate::model::song::Song;
use crate::*;
use super::super::read_file;

#[test]
fn test_gp4_slides() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/Slides.gp4")))
        .unwrap();
}

#[test]
fn test_gp4_legato_slide() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/legato-slide.gp4")))
        .unwrap();
}

#[test]
fn test_gp4_shift_slide() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/shift-slide.gp4")))
        .unwrap();
}

#[test]
fn test_gp4_slide_in_above() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/slide-in-above.gp4")))
        .unwrap();
}

#[test]
fn test_gp4_slide_in_below() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/slide-in-below.gp4")))
        .unwrap();
}

#[test]
fn test_gp4_slide_out_down() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/slide-out-down.gp4")))
        .unwrap();
}

#[test]
fn test_gp4_slide_out_up() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/slide-out-up.gp4")))
        .unwrap();
}

