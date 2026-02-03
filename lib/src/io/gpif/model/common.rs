use serde::Deserialize;

/// An empty self-closing tag used as a presence flag (e.g., `<Enable />`, `<LetRing />`).
#[derive(Debug, Deserialize)]
pub struct EnableTag;

#[derive(Debug, Deserialize)]
pub struct Property {
    #[serde(rename = "@name", default)]
    pub name: String,
    // Value sub-elements — each property uses at most one of these
    #[serde(rename = "Fret", default)]
    pub fret: Option<i32>,
    #[serde(rename = "String", default)]
    pub string: Option<i32>,
    #[serde(rename = "Pitch", default)]
    pub pitch: Option<Pitch>,
    #[serde(rename = "Number", default)]
    pub number: Option<i32>,
    #[serde(rename = "Enable", default)]
    pub enable: Option<EnableTag>,
    #[serde(rename = "Float", default)]
    pub float: Option<f64>,
    #[serde(rename = "Flags", default)]
    pub flags: Option<i32>,
    #[serde(rename = "HFret", default)]
    pub hfret: Option<f64>,
    #[serde(rename = "HType", default)]
    pub htype: Option<String>,
    #[serde(rename = "Pitches", default)]
    pub pitches: Option<String>,
    #[serde(rename = "Direction", default)]
    pub direction: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Pitch {
    #[serde(rename = "Step", default)]
    pub step: String,
    #[serde(rename = "Octave", default)]
    pub octave: i32,
    #[serde(rename = "Accidental", default)]
    pub accidental: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TieInfo {
    #[serde(rename = "@origin", default)]
    pub origin: String,
    #[serde(rename = "@destination", default)]
    pub destination: String,
}
