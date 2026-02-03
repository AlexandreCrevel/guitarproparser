use super::common::{EnableTag, Property, TieInfo};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NotesWrapper {
    #[serde(rename = "Note", default)]
    pub notes: Vec<Note>,
}

#[derive(Debug, Deserialize)]
pub struct Note {
    #[serde(rename = "@id", default)]
    pub id: i32,
    #[serde(rename = "Properties")]
    pub properties: NoteProperties,
    #[serde(rename = "Tie", default)]
    pub tie: Option<TieInfo>,
    #[serde(rename = "Vibrato", default)]
    pub vibrato: Option<String>,
    #[serde(rename = "LetRing", default)]
    pub let_ring: Option<EnableTag>,
    #[serde(rename = "AntiAccent", default)]
    pub anti_accent: Option<String>,
    #[serde(rename = "Accent", default)]
    pub accent: Option<i32>,
    #[serde(rename = "Trill", default)]
    pub trill: Option<i32>,
    #[serde(rename = "Ornament", default)]
    pub ornament: Option<String>,
    #[serde(rename = "LeftFingering", default)]
    pub left_fingering: Option<String>,
    #[serde(rename = "RightFingering", default)]
    pub right_fingering: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct NoteProperties {
    #[serde(rename = "Property", default)]
    pub properties: Vec<Property>,
}
