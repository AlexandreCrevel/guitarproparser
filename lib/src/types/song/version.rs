// Version structure for Guitar Pro files

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Version {
    pub data: String,
    pub number: (u8, u8, u8),
    pub clipboard: bool,
}
