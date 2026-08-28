use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Generation {
    pub number: u32,
    pub link: PathBuf,
    pub build_date: String,
    pub ver: String,
    pub kernel: String,
    pub is_current: bool,
}
