use std::fmt::Display;

use super::{PropOrIdx, SyntaxErr, ValidationErr};

#[derive(Debug, PartialEq)]
pub enum Err {
    SyntaxErr(SyntaxErr),
    ValidationErr(ValidationErr),
}

impl Err {
    pub fn new_validation_err<M: Display>(target: PropOrIdx, path: Vec<PropOrIdx>, msg: M) -> Self {
        Self::ValidationErr(ValidationErr {
            target,
            path,
            msg: msg.to_string(),
        })
    }
}

impl From<SyntaxErr> for Err {
    #[inline]
    fn from(value: SyntaxErr) -> Self {
        Self::SyntaxErr(value)
    }
}

impl From<ValidationErr> for Err {
    #[inline]
    fn from(value: ValidationErr) -> Self {
        Self::ValidationErr(value)
    }
}

#[derive(Debug, PartialEq)]
pub enum Errs {
    SyntaxErr(SyntaxErr),
    ValidationErrs(Vec<ValidationErr>),
}

impl From<SyntaxErr> for Errs {
    #[inline]
    fn from(value: SyntaxErr) -> Self {
        Errs::SyntaxErr(value)
    }
}

impl Errs {
    pub fn new_validation_err<M: ToString>(
        target: PropOrIdx,
        path: Vec<PropOrIdx>,
        msg: M,
    ) -> Self {
        Self::ValidationErrs(vec![ValidationErr {
            target,
            path,
            msg: msg.to_string(),
        }])
    }
}
