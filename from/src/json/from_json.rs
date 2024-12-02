use super::{Err, Errs, Path};
pub trait FromJson: FromJsonValue {
    fn from_json(json: &[u8]) -> Result<Self, Err> {
        let mut idx = 0usize;
        let path = crate::json::Path::new();
        Self::from_json_value(json, &mut idx, &path)
    }

    fn from_json_lang(json: &[u8], lang: &str) -> Result<Self, Err> {
        let mut idx = 0usize;
        let path = crate::json::Path::new();
        Self::from_json_value_lang(json, &mut idx, &path, lang)
    }

    fn from_json_stack_errs(json: &[u8]) -> Result<Self, Errs> {
        let mut idx = 0usize;
        let path = crate::json::Path::new();
        Self::from_json_value_stack_errs(json, &mut idx, &path)
    }
    fn from_json_stack_errs_lang(json: &[u8], lang: &str) -> Result<Self, Errs> {
        let mut idx = 0usize;
        let path = crate::json::Path::new();
        Self::from_json_value_stack_errs_lang(json, &mut idx, &path, lang)
    }
}

pub trait FromJsonValue: Sized {
    fn from_json_value(json: &[u8], idx: &mut usize, path: &Path) -> Result<Self, Err>;

    fn from_json_value_lang(
        json: &[u8],
        idx: &mut usize,
        path: &Path,
        lang: &str,
    ) -> Result<Self, Err>;

    fn from_json_value_stack_errs(json: &[u8], idx: &mut usize, path: &Path) -> Result<Self, Errs>;

    fn from_json_value_stack_errs_lang(
        json: &[u8],
        idx: &mut usize,
        path: &Path,
        lang: &str,
    ) -> Result<Self, Errs>;
}
