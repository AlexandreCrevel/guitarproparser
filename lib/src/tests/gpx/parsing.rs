// GPX parsing tests
use crate::model::song::Song;
use crate::*;
use super::super::{read_gpx, read_file};


#[test]
fn test_gpx_all_files_parse() {
    use std::fs;
    let test_dir = "../test";
    let mut pass = 0;
    let mut failures: Vec<String> = Vec::new();
    for entry in fs::read_dir(test_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().is_some_and(|e| e == "gpx") {
            let fname = path.file_name().unwrap().to_str().unwrap().to_string();
            let data = fs::read(&path).unwrap();
            let mut song = Song::default();
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                song.read_gpx(&data).unwrap();
            })) {
                Ok(_) => {
                    pass += 1;
                }
                Err(e) => {
                    let msg = if let Some(s) = e.downcast_ref::<String>() {
                        s.clone()
                    } else if let Some(s) = e.downcast_ref::<&str>() {
                        s.to_string()
                    } else {
                        "unknown".to_string()
                    };
                    let short = &msg[..msg.len().min(100)];
                    failures.push(format!("{}: {}", fname, short));
                }
            }
        }
    }
    if !failures.is_empty() {
        for f in &failures {
            eprintln!("FAIL: {}", f);
        }
    }
    eprintln!(
        "{} pass, {} fail out of {}",
        pass,
        failures.len(),
        pass + failures.len()
    );
    assert!(
        failures.is_empty(),
        "{} files failed to parse",
        failures.len()
    );
}

