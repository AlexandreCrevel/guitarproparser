use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TracksWrapper {
    #[serde(rename = "Track", default)]
    pub tracks: Vec<Track>,
}

#[derive(Debug, Deserialize)]
pub struct Track {
    #[serde(rename = "@id", default)]
    pub id: i32,
    #[serde(rename = "Name", default)]
    pub name: String,
    #[serde(rename = "ShortName", default)]
    pub short_name: String,
    #[serde(rename = "Color", default)]
    pub color: Option<String>,
    /// GP6: track-level properties (Tuning, DiagramCollection, etc.)
    #[serde(rename = "Properties", default)]
    pub properties: Option<TrackPropertiesWrapper>,
    /// GP7: staves with per-staff properties
    #[serde(rename = "Staves", default)]
    pub staves: Option<StavesWrapper>,
    #[serde(rename = "GeneralMidi", default)]
    pub general_midi: Option<GeneralMidi>,
    #[serde(rename = "Transpose", default)]
    pub transpose: Option<Transpose>,
    #[serde(rename = "RSE", default)]
    pub rse: Option<RseTrackWrapper>,
}

#[derive(Debug, Deserialize)]
pub struct RseTrackWrapper {
    #[serde(rename = "ChannelStrip", default)]
    pub channel_strip: Option<ChannelStrip>,
    #[serde(rename = "EffectChains", default)]
    pub effect_chains: Option<EffectChainsWrapper>,
}

#[derive(Debug, Deserialize)]
pub struct ChannelStrip {
    #[serde(rename = "Parameters", default)]
    pub parameters: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EffectChainsWrapper {
    #[serde(rename = "EffectChain", default)]
    pub effect_chains: Vec<EffectChain>,
}

#[derive(Debug, Deserialize)]
pub struct EffectChain {
    #[serde(rename = "Name", default)]
    pub name: String,
    #[serde(rename = "Rail", default)]
    pub rails: Vec<Rail>,
}

#[derive(Debug, Deserialize)]
pub struct Rail {
    #[serde(rename = "@name", default)]
    pub name: String,
    #[serde(rename = "Effect", default)]
    pub effects: Vec<RseEffect>,
}

#[derive(Debug, Deserialize)]
pub struct RseEffect {
    #[serde(rename = "@id", default)]
    pub id: String,
    #[serde(rename = "Parameters", default)]
    pub parameters: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TrackPropertiesWrapper {
    #[serde(rename = "Property", default)]
    pub properties: Vec<Property>,
}

// Property is also generic. Used in Note as well? Yes.
// I should probably make a `property.rs` or `common.rs`.
// For now I'll put it here and export it.

use super::common::Property;

#[derive(Debug, Deserialize)]
pub struct StavesWrapper {
    #[serde(rename = "Staff", default)]
    pub staves: Vec<Staff>,
}

#[derive(Debug, Deserialize)]
pub struct Staff {
    #[serde(rename = "Properties", default)]
    pub properties: Option<StaffPropertiesWrapper>,
}

#[derive(Debug, Deserialize)]
pub struct StaffPropertiesWrapper {
    #[serde(rename = "Property", default)]
    pub properties: Vec<Property>,
}

#[derive(Debug, Deserialize)]
pub struct GeneralMidi {
    #[serde(rename = "Program", default)]
    pub program: Option<i32>,
    #[serde(rename = "Port", default)]
    pub port: Option<i32>,
    #[serde(rename = "PrimaryChannel", default)]
    pub primary_channel: Option<i32>,
    #[serde(rename = "SecondaryChannel", default)]
    pub secondary_channel: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct Transpose {
    #[serde(rename = "Chromatic", default)]
    pub chromatic: Option<i32>,
    #[serde(rename = "Octave", default)]
    pub octave: Option<i32>,
}
