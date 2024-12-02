use super::{Path, ValidationErr};
pub trait Validator<V> {
    fn none(val: &V, path: &Path) -> Result<(), ValidationErr>;

    #[inline(always)]
    fn lang(val: &V, path: &Path, _: &str) -> Result<(), ValidationErr> {
        Self::none(val, path)
    }

    #[inline(always)]
    fn stack_errs(val: &V, path: &Path) -> Result<(), Vec<ValidationErr>> {
        match Self::none(val, path) {
            Ok(_) => Ok(()),

            Err(e) => Err(vec![e]),
        }
    }

    #[inline(always)]
    fn stack_errs_lang(val: &V, path: &Path, _: &str) -> Result<(), Vec<ValidationErr>> {
        match Self::none(val, path) {
            Ok(_) => Ok(()),

            Err(e) => Err(vec![e]),
        }
    }
}
