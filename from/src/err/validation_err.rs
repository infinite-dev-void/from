use super::{Path, PropOrIdx};

#[derive(Clone, Debug, PartialEq)]
pub struct ValidationErr {
    pub target: PropOrIdx,
    pub path: Path,
    pub msg: String,
}

impl ValidationErr {
    #[inline]
    pub fn new(target: PropOrIdx, path: Path, msg: &str) -> Self {
        Self {
            target,
            path,
            msg: String::from(msg),
        }
    }
}
