// GPX ottava tests
use super::super::{read_gpx};


#[test]
fn test_gpx_ottava1() {
    let song = read_gpx("test/ottava1.gpx");
    assert!(!song.tracks.is_empty());
}

#[test]
fn test_gpx_ottava2() {
    let song = read_gpx("test/ottava2.gpx");
    assert!(!song.tracks.is_empty());
}

#[test]
fn test_gpx_ottava3() {
    let song = read_gpx("test/ottava3.gpx");
    assert!(!song.tracks.is_empty());
}

#[test]
fn test_gpx_ottava4() {
    let song = read_gpx("test/ottava4.gpx");
    assert!(!song.tracks.is_empty());
}

#[test]
fn test_gpx_ottava5() {
    let song = read_gpx("test/ottava5.gpx");
    assert!(!song.tracks.is_empty());
}

