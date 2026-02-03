// Marker structure for Guitar Pro measures

/// A marker annotation for beats.
#[derive(Debug, Clone)]
pub struct Marker {
    pub title: String,
    pub color: i32,
}

impl Default for Marker {
    fn default() -> Self {
        Marker {
            title: "Section".to_owned(),
            color: 0xff0000,
        }
    }
}
