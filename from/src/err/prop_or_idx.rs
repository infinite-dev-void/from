#[derive(Clone, Debug, PartialEq)]
pub enum PropOrIdx {
    Prop(String),
    Idx(usize),
}

impl PropOrIdx {
    #[inline]
    pub fn to_json(&self) -> String {
        match self {
            PropOrIdx::Prop(p) => String::from('"') + &p + "\"",
            PropOrIdx::Idx(i) => i.to_string(),
        }
    }
}

impl From<&str> for PropOrIdx {
    #[inline]
    fn from(value: &str) -> Self {
        Self::Prop(value.to_owned())
    }
}

impl From<usize> for PropOrIdx {
    #[inline]
    fn from(value: usize) -> Self {
        Self::Idx(value)
    }
}

/* impl From<&[u8]> for PropOrIdx {
    #[inline]
    fn from(value: &[u8]) -> Self {
        Self::Prop(unsafe { String::from_utf8_unchecked(Vec::from(value)) })
    }
} */

pub type Path = Vec<PropOrIdx>;
