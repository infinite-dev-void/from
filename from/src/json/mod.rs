// mod prop_or_idx;
// pub use prop_or_idx::{Path, PropOrIdx};
//
// mod syntx_err;
// pub use syntx_err::SyntaxErr;
//
// mod validation_err;
// pub use validation_err::ValidationErr;
//
// mod err;
// pub use err::{Err, Errs};

use super::{Err, Errs, Path, SyntaxErr};

pub mod utils;

/* pub mod parse;
pub use parse::{BoolParseResult, FloatParseResult, IntParseResult, StringParseResult}; */

pub mod string;

mod ints;
pub use ints::{i128, i16, i32, i64, i8, isize, u128, u16, u32, u64, u8, usize};

pub mod bool;

pub mod prop;

pub mod float;
pub use float::{f32, f64};

pub mod object;
pub mod vec;

mod from_json;
pub use from_json::{FromJson, FromJsonValue};
