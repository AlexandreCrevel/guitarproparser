// GP7 fermata tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gp7, read_file};


#[test]
fn test_gp7_fermata() {
    use crate::model::headers::FermataType;
    let song = read_gp7("test/fermata.gp");
    assert!(!song.tracks.is_empty());
    let has_fermata = song
        .measure_headers
        .iter()
        .any(|mh| !mh.fermatas.is_empty());
    assert!(has_fermata, "fermata.gp should contain at least one fermata");
    let first_fermata = song
        .measure_headers
        .iter()
        .flat_map(|mh| mh.fermatas.iter())
        .next()
        .expect("should have a fermata");
    assert_eq!(first_fermata.fermata_type, FermataType::Medium);
    assert_ne!(first_fermata.offset.1, 0, "fermata offset denominator should not be zero");
}

