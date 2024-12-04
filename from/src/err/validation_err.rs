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

    #[inline]
    pub fn to_json(&self) -> String {
        let mut json = String::from(r#"{"target":"#);
        json.push_str(&self.target.to_json());
        json.push(',');
        json.push_str(r#""path":"#);
        json.push_str(&path_to_json(&self.path));
        json.push(',');
        json.push_str(r#""msg":"#);
        json.push_str(&super::string_to_json(&self.msg));
        json.push('}');

        json
    }
}

pub fn path_to_json(path: &Path) -> String {
    if path.len() == 0 {
        return String::from("[]");
    };

    let mut json = String::from('[');

    let mut i = 0;

    while path.len() - 1 > i {
        json.push_str(&path[i].to_json());
        json.push(',');
        i += 1;
    }

    json.push_str(&path[i].to_json());

    json.push(']');

    json
}
