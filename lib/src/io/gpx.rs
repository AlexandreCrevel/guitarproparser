use std::io::{Read, Cursor};
use zip::ZipArchive;
use crate::io::gpif::Gpif;
use quick_xml::de::from_str;

/// Reads a .gp (GP7+) file which is a ZIP archive containing 'Content/score.gpif'.
pub fn read_gp(data: &[u8]) -> Result<Gpif, String> {
    let cursor = Cursor::new(data);
    let mut zip = ZipArchive::new(cursor).map_err(|e| format!("Zip error: {}", e))?;

    // Standard path for GP7 files
    let mut file = zip.by_name("Content/score.gpif").map_err(|e| format!("Could not find score.gpif: {}", e))?;
    
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| format!("Read error: {}", e))?;

    let gpif: Gpif = from_str(&contents).map_err(|e| format!("XML Parse error: {}", e))?;
    Ok(gpif)
}
