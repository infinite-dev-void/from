use std::ops::RangeInclusive;

use crate::json::utils::get_or_unexpected_end;

use super::{utils, SyntaxErr};

pub const DIGITS: RangeInclusive<u8> = b'0'..=b'9';

#[derive(Debug, PartialEq, Eq)]
pub enum ParseResult<T> {
    Ok(T),
    Null,
    TypeMismatch(String),            // expected, found
    TooLargeToFitInto(&'static str), // target type
    TooSmallToFitInto(&'static str), // target type
    SyntaxErr(super::SyntaxErr),
}

impl<T> ParseResult<T> {
    #[inline]
    fn syntax_err(msg: &str, idx: &mut usize) -> Self {
        Self::SyntaxErr(SyntaxErr::new(msg, idx))
    }

    #[inline]
    fn unexpected_token(exp: &str, found: u8, idx: &mut usize) -> Self {
        Self::SyntaxErr(SyntaxErr::unexpected_token(exp, &[found], idx))
    }

    #[inline]
    fn type_mismatch(found: &str) -> Self {
        ParseResult::TypeMismatch(String::from(found))
    }

    #[inline]
    pub fn unwrap(self) -> T {
        match self {
            Self::Ok(val) => val,
            _ => panic!(),
        }
    }
}

#[inline]
fn skip_and_return_too_small_err<T>(
    json: &[u8],
    idx: &mut usize,
    ty: &'static str,
) -> ParseResult<T> {
    let mut byte;

    loop {
        byte = match json.get(*idx) {
            Some(byte) => *byte,
            None => break,
        };

        match byte {
            b'0'..=b'9' => {
                *idx += 1;
                continue;
            }

            b'.' => {
                return skip_and_return_mismatch_ty(json, idx);
            }

            b',' | b' ' | b'}' | b']' | b'\n' | b'\r' | b'\t' => {
                break;
            }

            _ => return ParseResult::unexpected_token("digit", byte, idx),
        };
    }

    ParseResult::TooSmallToFitInto(ty)
}

#[inline]
fn skip_and_return_too_large_err<T>(
    json: &[u8],
    idx: &mut usize,
    ty: &'static str,
) -> ParseResult<T> {
    let mut byte;
    loop {
        byte = match json.get(*idx) {
            Some(byte) => *byte,
            None => break,
        };

        match byte {
            b'0'..=b'9' => {
                *idx += 1;
                continue;
            }

            b'.' => {
                return skip_and_return_mismatch_ty(json, idx);
            }

            b',' | b' ' | b'}' | b']' | b'\n' | b'\r' | b'\t' => {
                break;
            }

            _ => return ParseResult::unexpected_token("digit", byte, idx),
        };
    }

    ParseResult::TooLargeToFitInto(ty)
}

// This function will be called when a dot is encountered
#[inline]
fn skip_and_return_mismatch_ty<T>(json: &[u8], idx: &mut usize) -> ParseResult<T> {
    let mut byte = match get_or_unexpected_end(json, idx) {
        Ok(byte) => byte,
        Err(e) => return ParseResult::SyntaxErr(e),
    };

    loop {
        match byte {
            b'0'..=b'9' => {
                *idx += 1;
            }

            b',' | b' ' | b'}' | b']' | b'\n' | b'\r' | b'\t' => {
                break;
            }

            _ => return ParseResult::unexpected_token("digit", byte, idx),
        };

        byte = match json.get(*idx) {
            Some(byte) => *byte,
            None => break,
        };
    }

    ParseResult::type_mismatch("float")
}

macro_rules! define_signed_parser_and_test {
    (
        typ = $typ:ident,
        neg_typ = $neg_typ:ident,
        pos_typ = $pos_typ:ident,
        typ_str = $typ_str:expr,
        repeat = $repeat:expr,
        max_safe_num_before_push = $max_safe_num_before_push: expr,
        less_tpy_min_str = $less_typ_min_str: expr,
        greater_typ_max_str = $greater_typ_max_str: expr,
        test_step_by = $test_step_by: expr,
    ) => {
        pub mod $typ {
            use super::{
                skip_and_return_mismatch_ty, skip_and_return_too_large_err,
                skip_and_return_too_small_err, utils, DIGITS,
            };

            pub type ParseResult = super::ParseResult<$typ>;

            struct NumRange {
                current: $typ,
                overflowed: bool,
                step: $typ,
            }

            impl NumRange {
                #[allow(dead_code)]
                fn new(start: $typ, step: $typ) -> Self {
                    Self {
                        current: start,
                        overflowed: false,
                        step,
                    }
                }
            }

            impl Iterator for NumRange {
                type Item = $typ;
                fn next(&mut self) -> Option<Self::Item> {
                    if self.overflowed {
                        return None;
                    };

                    let c = self.current;
                    (self.current, self.overflowed) = self.current.overflowing_add(self.step);

                    return Some(c);
                }
            }

            const TY: &str = $typ_str;

            pub fn parse(json: &[u8], idx: &mut usize) -> ParseResult {
                let byte = match utils::get_or_unexpected_end(json, idx) {
                    Ok(byte) => byte,
                    Err(e) => return ParseResult::SyntaxErr(e),
                };

                match byte {
                    b'-' => {
                        *idx += 1;
                        $neg_typ(json, idx)
                    }
                    // b'+' =>{} not in https://www.json.org
                    b'0'..=b'9' => {
                        *idx += 1;
                        $pos_typ(json, idx, byte.wrapping_sub(48) as $typ)
                    }

                    // null
                    b'n' => {
                        if let Err(e) = utils::skip_null(json, idx, $typ_str) {
                            ParseResult::SyntaxErr(e)
                        } else {
                            ParseResult::Null
                        }
                    }

                    // type mismatch
                    b'"' => {
                        if let Err(e) = utils::skip_string(json, idx) {
                            return ParseResult::SyntaxErr(e);
                        };

                        return ParseResult::type_mismatch("string");
                    }

                    b'f' => {
                        if let Err(e) = utils::skip_false(json, idx, $typ_str) {
                            ParseResult::SyntaxErr(e)
                        } else {
                            ParseResult::type_mismatch("boolean")
                        }
                    }

                    b't' => {
                        if let Err(e) = utils::skip_true(json, idx, $typ_str) {
                            ParseResult::SyntaxErr(e)
                        } else {
                            ParseResult::type_mismatch("boolean")
                        }
                    }

                    b'{' => {
                        if let Err(e) = utils::skip_object(json, idx) {
                            ParseResult::SyntaxErr(e)
                        } else {
                            ParseResult::type_mismatch("object")
                        }
                    }

                    b'[' => {
                        if let Err(e) = utils::skip_array(json, idx) {
                            ParseResult::SyntaxErr(e)
                        } else {
                            ParseResult::type_mismatch("array")
                        }
                    }
                    _ => ParseResult::syntax_err("invalid digit", idx),
                }
            }

            #[inline(always)]
            pub fn $neg_typ(json: &[u8], idx: &mut usize) -> ParseResult {
                let mut byte = match utils::get_or_unexpected_end(json, idx) {
                    Ok(byte) => byte,
                    Err(e) => return ParseResult::SyntaxErr(e),
                };

                // must begin with digit
                if !DIGITS.contains(&byte) {
                    return ParseResult::unexpected_token("digit", byte, idx);
                };

                let mut num = -(byte.wrapping_sub(48) as $typ);

                //
                //
                //
                //
                //
                //
                //
                // It is not possible for the number to overflow
                // within the following range
                ::loop_code::repeat!($repeat {
                    *idx += 1;

                    // may be it is a standalone value so there is no need to return
                    // an error
                    byte = match json.get(*idx) {
                        Some(byte) => *byte,
                        None => return ParseResult::Ok(num),
                    };

                    match byte {
                        b'0'..=b'9' => {
                            // add zero to the right
                            // optimization: these methods will not check for
                            // the overflow.
                            num = num
                                .wrapping_mul(10)
                                .wrapping_sub(byte.wrapping_sub(48) as $typ);
                        }

                        // it is not the responsibilty of the parser to validate
                        // the possible ends of the value
                        b',' | b' ' | b'}' | b']' | b'\n' | b'\r' | b'\t' => {
                            return ParseResult::Ok(num)
                        }

                        b'.' => {
                            *idx += 1;
                            return skip_and_return_mismatch_ty(json, idx);
                        }

                        _ => return ParseResult::syntax_err("invalid digit", idx),
                    };
                });

                //
                //
                //
                //
                //
                //
                //
                //
                // Handle the last digit that may cause an overflow

                *idx += 1;

                // may it is a standalone value so there is no need to return
                // an error
                byte = match json.get(*idx) {
                    Some(byte) => *byte,
                    None => return ParseResult::Ok(num),
                };

                match byte {
                    b'0'..=b'9' => {
                        if num < -$max_safe_num_before_push {
                            return skip_and_return_too_small_err(json, idx, TY);
                        };
                        num = num.wrapping_mul(10);

                        let overflow;
                        (num, overflow) = num.overflowing_sub(byte.wrapping_sub(48) as $typ);

                        if overflow {
                            return skip_and_return_too_small_err(json, idx, TY);
                        };
                    }

                    // it is not the responsibilty of the parser to validate
                    // the possible ends of the value
                    b',' | b' ' | b'}' | b']' | b'\n' | b'\r' | b'\t' => {
                        return ParseResult::Ok(num)
                    }

                    b'.' => {}

                    _ => return ParseResult::syntax_err("invalid digit", idx),
                };

                //
                //
                //
                //
                // check if there is more digits and so return an error
                // because digit will cause an overflow

                *idx += 1;

                // may it is a standalone value so there is no need to return
                // an error
                byte = match json.get(*idx) {
                    Some(byte) => *byte,
                    None => return ParseResult::Ok(num),
                };

                match byte {
                    b',' | b' ' | b'}' | b']' | b'\n' | b'\r' | b'\t' => {
                        return ParseResult::Ok(num)
                    }

                    b'0'..=b'9' => return skip_and_return_too_small_err(json, idx, TY),

                    b'.' => {
                        *idx += 1;
                        return skip_and_return_mismatch_ty(json, idx);
                    }

                    _ => return ParseResult::syntax_err("invalid digit", idx),
                }
            }

            #[inline(always)]
            pub fn $pos_typ(json: &[u8], idx: &mut usize, mut num: $typ) -> ParseResult {
                let mut byte;

                // overflow will not be occur within the following range
                ::loop_code::repeat!($repeat {
                    byte = match json.get(*idx) {
                        Some(byte) => *byte,
                        None => return ParseResult::Ok(num),
                    };

                    match byte {
                        b'0'..=b'9' => {
                            num = num
                                .wrapping_mul(10)
                                .wrapping_add(byte.wrapping_sub(48) as $typ);
                        }

                        b',' | b' ' | b'}' | b']' | b'\n' | b'\r' | b'\t' => {
                            return ParseResult::Ok(num)
                        }

                        b'.' => {
                            *idx += 1;
                            return skip_and_return_mismatch_ty(json, idx);
                        }

                        _ => return ParseResult::unexpected_token("digit", byte, idx),
                    }

                    *idx += 1;
                });


                // overflow may occur
                byte = match json.get(*idx) {
                    Some(byte) => *byte,
                    None => return ParseResult::Ok(num),
                };

                match byte {
                    b'0'..=b'9' => {
                        if num > $max_safe_num_before_push {
                            return skip_and_return_too_large_err(json, idx, TY);
                        };
                        num = num.wrapping_mul(10);
                        let overflow;
                        (num, overflow) = num.overflowing_add(byte.wrapping_sub(48) as $typ);

                        if overflow {
                            return skip_and_return_too_large_err(json, idx, TY);
                        };
                    }

                    b',' | b' ' | b'}' | b']' | b'\n' | b'\r' | b'\t' => {
                        return ParseResult::Ok(num)
                    }

                    b'.' => {
                        *idx += 1;
                        return skip_and_return_mismatch_ty(json, idx);
                    }

                    _ => return ParseResult::unexpected_token("digit", byte, idx),
                }

                //
                //
                //
                //
                //
                // byte that will cause an overflow if it is digit

                *idx += 1;
                byte = match json.get(*idx) {
                    Some(byte) => *byte,
                    None => return ParseResult::Ok(num),
                };

                match byte {
                    b'0'..=b'9' => {
                        *idx += 1;
                        skip_and_return_too_large_err(json, idx, TY)
                    }

                    b',' | b' ' | b'}' | b']' | b'\n' | b'\r' | b'\t' => ParseResult::Ok(num),

                    b'.' => {
                        *idx += 1;
                        skip_and_return_mismatch_ty(json, idx)
                    }

                    _ => ParseResult::unexpected_token("digit", byte, idx),
                }
            }

            #[cfg(test)]
            mod test {
                #[test]
                fn valid() {
                    for i in super::NumRange::new($typ::MIN, $test_step_by) {
                        assert_eq!(
                            super::parse(i.to_string().as_bytes(), &mut 0),
                            super::ParseResult::Ok(i)
                        );
                    }
                }

                #[test]
                fn too_small() {
                    assert_eq!(
                        super::parse($less_typ_min_str.as_bytes(), &mut 0),
                        super::ParseResult::TooSmallToFitInto($typ_str)
                    )
                }

                #[test]
                fn too_large() {
                    assert_eq!(
                        super::parse($greater_typ_max_str.as_bytes(), &mut 0),
                        super::ParseResult::TooLargeToFitInto($typ_str)
                    )
                }

                #[test]
                fn type_mismatch_float() {
                    assert_eq!(
                        super::parse("12.8".as_bytes(), &mut 0),
                        super::ParseResult::type_mismatch("float")
                    )
                }

                #[test]
                fn type_mismatch_string() {
                    assert_eq!(
                        super::parse(r#""12.8""#.as_bytes(), &mut 0),
                        super::ParseResult::type_mismatch("string")
                    )
                }

                #[test]
                fn type_mismatch_bool_false() {
                    assert_eq!(
                        super::parse(r#"false"#.as_bytes(), &mut 0),
                        super::ParseResult::type_mismatch("boolean")
                    )
                }

                #[test]
                fn type_mismatch_bool_true() {
                    assert_eq!(
                        super::parse(r#"true"#.as_bytes(), &mut 0),
                        super::ParseResult::type_mismatch("boolean")
                    )
                }

                #[test]
                fn type_mismatch_array() {
                    assert_eq!(
                        super::parse(r#"[]"#.as_bytes(), &mut 0),
                        super::ParseResult::type_mismatch("array")
                    )
                }

                #[test]
                fn type_mismatch_object() {
                    assert_eq!(
                        super::parse(r#"{}"#.as_bytes(), &mut 0),
                        super::ParseResult::type_mismatch("object")
                    )
                }

                #[test]
                fn null() {
                    assert_eq!(
                        super::parse(r#"null"#.as_bytes(), &mut 0),
                        super::ParseResult::Null
                    )
                }

                #[test]
                fn syntax_err() {
                    assert_eq!(
                        super::parse("12.8.".as_bytes(), &mut 0),
                        super::ParseResult::syntax_err("expected: digit, found: .", &mut 4),
                    )
                }
            }
        }
    };
}

define_signed_parser_and_test!(
    typ = i8,
    neg_typ = neg_i8,
    pos_typ = pos_i8,
    typ_str = "i8",
    repeat = 1,
    max_safe_num_before_push = 12,
    less_tpy_min_str = "-129",
    greater_typ_max_str = "128",
    test_step_by = 1,
);

define_signed_parser_and_test!(
    typ = i16,
    neg_typ = neg_i16,
    pos_typ = pos_i16,
    typ_str = "i16",
    repeat = 3,
    max_safe_num_before_push = 3276,
    less_tpy_min_str = "-32769",
    greater_typ_max_str = "32768",
    test_step_by = 255,
);

define_signed_parser_and_test!(
    typ = i32,
    neg_typ = neg_i32,
    pos_typ = pos_i32,
    typ_str = "i32",
    repeat = 8,
    max_safe_num_before_push = 214748364,
    less_tpy_min_str = "-2147483649",
    greater_typ_max_str = "2147483648",
    test_step_by = 8589934,
);

define_signed_parser_and_test!(
    typ = i64,
    neg_typ = neg_i64,
    pos_typ = pos_i64,
    typ_str = "i64",
    repeat = 17,
    max_safe_num_before_push = 922337203685477580,
    less_tpy_min_str = "-9223372036854775809",
    greater_typ_max_str = "9223372036854775808",
    test_step_by = 36170086419038336,
);

define_signed_parser_and_test!(
    typ = i128,
    neg_typ = neg_i128,
    pos_typ = pos_i128,
    typ_str = "i128",
    repeat = 37,
    max_safe_num_before_push = 17014118346046923173168730371588410572,
    less_tpy_min_str = "-170141183460469231731687303715884105729",
    greater_typ_max_str = "170141183460469231731687303715884105728",
    test_step_by = 667220327295957771496812955748565120,
);

#[cfg(target_pointer_width = "32")]
define_signed_parser_and_test!(
    typ = isize,
    neg_typ = neg_isize,
    pos_typ = pos_isize,
    typ_str = "isize",
    repeat = 8,
    max_safe_num_before_push = 214748364,
    less_tpy_min_str = "-2147483649",
    greater_typ_max_str = "2147483648",
    test_step_by = 8589934,
);

#[cfg(target_pointer_width = "64")]
define_signed_parser_and_test!(
    typ = isize,
    neg_typ = neg_isize,
    pos_typ = pos_isize,
    typ_str = "isize",
    repeat = 17,
    max_safe_num_before_push = 922337203685477580,
    less_tpy_min_str = "-9223372036854775809",
    greater_typ_max_str = "9223372036854775808",
    test_step_by = 36170086419038336,
);

macro_rules! define_unsigned_parser_and_test {
    (
        typ = $typ:ident,
        neg_typ = $neg_typ:ident,
        pos_typ = $pos_typ:ident,
        typ_str = $typ_str:expr,
        repeat = $repeat:expr,
        max_safe_num_before_push = $max_safe_num_before_push: expr,
        less_tpy_min_str = $less_typ_min_str: expr,
        greater_typ_max_str = $greater_typ_max_str: expr,
        test_step_by = $test_step_by: expr,
    ) => {
        pub mod $typ {
            use super::{
                skip_and_return_mismatch_ty, skip_and_return_too_large_err,
                skip_and_return_too_small_err, utils, DIGITS,
            };

            pub type ParseResult = super::ParseResult<$typ>;

            struct NumRange {
                current: $typ,
                overflowed: bool,
                step: $typ,
            }

            impl NumRange {
                #[allow(dead_code)]
                fn new(start: $typ, step: $typ) -> Self {
                    Self {
                        current: start,
                        overflowed: false,
                        step,
                    }
                }
            }

            impl Iterator for NumRange {
                type Item = $typ;
                fn next(&mut self) -> Option<Self::Item> {
                    if self.overflowed {
                        return None;
                    };

                    let c = self.current;
                    (self.current, self.overflowed) = self.current.overflowing_add(self.step);

                    return Some(c);
                }
            }

            const TY: &str = $typ_str;

            pub fn parse(json: &[u8], idx: &mut usize) -> ParseResult {
                let byte = match utils::get_or_unexpected_end(json, idx) {
                    Ok(byte) => byte,
                    Err(e) => return ParseResult::SyntaxErr(e),
                };

                match byte {
                    // b'+' =>{} not in https://www.json.org
                    b'0'..=b'9' => {
                        *idx += 1;
                        $pos_typ(json, idx, byte.wrapping_sub(48) as $typ)
                    }

                    b'-' => {
                        *idx += 1;
                        $neg_typ(json, idx)
                    }

                    // null
                    b'n' => {
                        if let Err(e) = utils::skip_null(json, idx, $typ_str) {
                            ParseResult::SyntaxErr(e)
                        } else {
                            ParseResult::Null
                        }
                    }

                    // type mismatch
                    b'"' => {
                        if let Err(e) = utils::skip_string(json, idx) {
                            return ParseResult::SyntaxErr(e);
                        };

                        return ParseResult::type_mismatch("string");
                    }

                    b'f' => {
                        if let Err(e) = utils::skip_false(json, idx, $typ_str) {
                            ParseResult::SyntaxErr(e)
                        } else {
                            ParseResult::type_mismatch("boolean")
                        }
                    }

                    b't' => {
                        if let Err(e) = utils::skip_true(json, idx, $typ_str) {
                            ParseResult::SyntaxErr(e)
                        } else {
                            ParseResult::type_mismatch("boolean")
                        }
                    }

                    b'{' => {
                        if let Err(e) = utils::skip_object(json, idx) {
                            ParseResult::SyntaxErr(e)
                        } else {
                            ParseResult::type_mismatch("object")
                        }
                    }

                    b'[' => {
                        if let Err(e) = utils::skip_array(json, idx) {
                            ParseResult::SyntaxErr(e)
                        } else {
                            ParseResult::type_mismatch("array")
                        }
                    }
                    _ => ParseResult::syntax_err("invalid digit", idx),
                }
            }

            #[inline(always)]
            pub fn $neg_typ(json: &[u8], idx: &mut usize) -> ParseResult {
                let mut byte = match utils::get_or_unexpected_end(json, idx) {
                    Ok(byte) => byte,
                    Err(e) => return ParseResult::SyntaxErr(e),
                };

                // must begin with digit
                if !DIGITS.contains(&byte) {
                    return ParseResult::unexpected_token("digit", byte, idx);
                };

                let mut num: $typ = 0;
                let mut overflow;

                (num, overflow) = num.overflowing_sub(byte.wrapping_sub(48) as $typ);

                if overflow {
                    return skip_and_return_too_small_err(json, idx, TY);
                };

                loop {
                    *idx += 1;

                    byte = match json.get(*idx) {
                        Some(byte) => *byte,
                        None => return ParseResult::Ok(num),
                    };

                    match byte {
                        b'0'..=b'9' => {
                            // there is no need to multiply by
                            // because 0 * 10 = 0
                            // num = num.wrapped_mul(10);

                            (num, overflow) = num.overflowing_sub(byte.wrapping_sub(48) as $typ);

                            if overflow {
                                return skip_and_return_too_small_err(json, idx, TY);
                            };
                        }

                        b',' | b' ' | b'}' | b']' | b'\n' | b'\r' | b'\t' => {
                            return ParseResult::Ok(num)
                        }

                        b'.' => {
                            *idx += 1;
                            return skip_and_return_mismatch_ty(json, idx);
                        }

                        _ => return ParseResult::unexpected_token("digit", byte, idx),
                    }
                }
            }

            #[inline(always)]
            pub fn $pos_typ(json: &[u8], idx: &mut usize, mut num: $typ) -> ParseResult {
                let mut byte;

                ::loop_code::repeat!($repeat {
                    byte = match json.get(*idx) {
                        Some(byte) => *byte,
                        None => return ParseResult::Ok(num),
                    };

                    match byte {
                        b'0'..=b'9' => {
                            num = num
                                .wrapping_mul(10)
                                .wrapping_add(byte.wrapping_sub(48) as $typ);
                        }

                        b',' | b' ' | b'}' | b']' | b'\n' | b'\r' | b'\t' => {
                            return ParseResult::Ok(num)
                        }

                        b'.' => {
                            *idx += 1;
                            return skip_and_return_mismatch_ty(json, idx);
                        }

                        _ => return ParseResult::unexpected_token("digit", byte, idx),
                    }

                    *idx += 1;
                });
                // overflow will not be occur within the following range
                /* for _ in $repeat {
                    byte = match json.get(*idx) {
                        Some(byte) => *byte,
                        None => return ParseResult::Ok(num),
                    };

                    match byte {
                        b'0'..=b'9' => {
                            num = num
                                .wrapping_mul(10)
                                .wrapping_add(byte.wrapping_sub(48) as $typ);
                        }

                        b',' | b' ' | b'}' | b']' | b'\n' | b'\r' | b'\t' => {
                            return ParseResult::Ok(num)
                        }

                        b'.' => {
                            *idx += 1;
                            return skip_and_return_mismatch_ty(json, idx);
                        }

                        _ => return ParseResult::unexpected_token("digit", byte, idx),
                    }

                    *idx += 1;
                } */

                // overflow may occur
                byte = match json.get(*idx) {
                    Some(byte) => *byte,
                    None => return ParseResult::Ok(num),
                };

                match byte {
                    b'0'..=b'9' => {
                        if num > $max_safe_num_before_push {
                            return skip_and_return_too_large_err(json, idx, TY);
                        };

                        let overflow;

                        (num, overflow) = num
                            .wrapping_mul(10)
                            .overflowing_add(byte.wrapping_sub(48) as $typ);

                        if overflow {
                            return skip_and_return_too_large_err(json, idx, TY);
                        };
                    }

                    b',' | b' ' | b'}' | b']' | b'\n' | b'\r' | b'\t' => {
                        return ParseResult::Ok(num)
                    }

                    b'.' => {
                        *idx += 1;
                        return skip_and_return_mismatch_ty(json, idx);
                    }

                    _ => return ParseResult::unexpected_token("digit", byte, idx),
                }

                //
                //
                //
                //
                //
                // byte that will cause an overflow if it is digit

                *idx += 1;
                byte = match json.get(*idx) {
                    Some(byte) => *byte,
                    None => return ParseResult::Ok(num),
                };

                match byte {
                    b'0'..=b'9' => {
                        *idx += 1;
                        skip_and_return_too_large_err(json, idx, TY)
                    }

                    b',' | b' ' | b'}' | b']' | b'\n' | b'\r' | b'\t' => ParseResult::Ok(num),

                    b'.' => {
                        *idx += 1;
                        skip_and_return_mismatch_ty(json, idx)
                    }

                    _ => ParseResult::unexpected_token("digit", byte, idx),
                }
            }

            #[cfg(test)]
            mod test {
                #[test]
                fn valid() {
                    for i in super::NumRange::new($typ::MIN, $test_step_by) {
                        assert_eq!(
                            super::parse(i.to_string().as_bytes(), &mut 0),
                            super::ParseResult::Ok(i)
                        );
                    }
                }

                #[test]
                fn special_case() {
                    assert_eq!(
                        // any number of zeros
                        super::parse("-000".as_bytes(), &mut 0),
                        super::ParseResult::Ok(0)
                    );
                }

                #[test]
                fn too_small() {
                    assert_eq!(
                        super::parse($less_typ_min_str.as_bytes(), &mut 0),
                        super::ParseResult::TooSmallToFitInto($typ_str)
                    )
                }

                #[test]
                fn too_large() {
                    assert_eq!(
                        super::parse($greater_typ_max_str.as_bytes(), &mut 0),
                        super::ParseResult::TooLargeToFitInto($typ_str)
                    )
                }

                #[test]
                fn type_mismatch_float() {
                    assert_eq!(
                        super::parse("12.8".as_bytes(), &mut 0),
                        super::ParseResult::type_mismatch("float")
                    )
                }

                #[test]
                fn type_mismatch_string() {
                    assert_eq!(
                        super::parse(r#""12.8""#.as_bytes(), &mut 0),
                        super::ParseResult::type_mismatch("string")
                    )
                }

                #[test]
                fn type_mismatch_bool_false() {
                    assert_eq!(
                        super::parse(r#"false"#.as_bytes(), &mut 0),
                        super::ParseResult::type_mismatch("boolean")
                    )
                }

                #[test]
                fn type_mismatch_bool_true() {
                    assert_eq!(
                        super::parse(r#"true"#.as_bytes(), &mut 0),
                        super::ParseResult::type_mismatch("boolean")
                    )
                }

                #[test]
                fn type_mismatch_array() {
                    assert_eq!(
                        super::parse(r#"[]"#.as_bytes(), &mut 0),
                        super::ParseResult::type_mismatch("array")
                    )
                }

                #[test]
                fn type_mismatch_object() {
                    assert_eq!(
                        super::parse(r#"{}"#.as_bytes(), &mut 0),
                        super::ParseResult::type_mismatch("object")
                    )
                }

                #[test]
                fn null() {
                    assert_eq!(
                        super::parse(r#"null"#.as_bytes(), &mut 0),
                        super::ParseResult::Null
                    )
                }

                #[test]
                fn syntax_err() {
                    assert_eq!(
                        super::parse("12.8.".as_bytes(), &mut 0),
                        super::ParseResult::syntax_err("expected: digit, found: .", &mut 4),
                    )
                }
            }
        }
    };
}

define_unsigned_parser_and_test!(
    typ = u8,
    neg_typ = neg_u8,
    pos_typ = pos_u8,
    typ_str = "u8",
    repeat = 1,
    max_safe_num_before_push = 25,
    less_tpy_min_str = "-1",
    greater_typ_max_str = "256",
    test_step_by = 1,
);

define_unsigned_parser_and_test!(
    typ = u16,
    neg_typ = neg_u32,
    pos_typ = pos_u32,
    typ_str = "u16",
    repeat = 3,
    max_safe_num_before_push = 6553,
    less_tpy_min_str = "-1",
    greater_typ_max_str = "65536",
    test_step_by = 255,
);

define_unsigned_parser_and_test!(
    typ = u32,
    neg_typ = neg_u32,
    pos_typ = pos_u32,
    typ_str = "u32",
    repeat = 8,
    max_safe_num_before_push = 429496729,
    less_tpy_min_str = "-1",
    greater_typ_max_str = "4294967296",
    test_step_by = 8589934,
);

define_unsigned_parser_and_test!(
    typ = u64,
    neg_typ = neg_u64,
    pos_typ = pos_u64,
    typ_str = "u64",
    repeat = 18,
    max_safe_num_before_push = 1844674407370955161,
    less_tpy_min_str = "-1",
    greater_typ_max_str = "18446744073709551616",
    test_step_by = 36170086419038336,
);

define_unsigned_parser_and_test!(
    typ = u128,
    neg_typ = neg_u128,
    pos_typ = pos_u128,
    typ_str = "u128",
    repeat = 37,
    max_safe_num_before_push = 34028236692093846346337460743176821145,
    less_tpy_min_str = "-1",
    greater_typ_max_str = "340282366920938463463374607431768211456",
    test_step_by = 667220327295957771496812955748565120,
);

#[cfg(target_pointer_width = "32")]
define_unsigned_parser_and_test!(
    typ = usize,
    neg_typ = neg_usize,
    pos_typ = pos_usize,
    typ_str = "usize",
    repeat = 8,
    max_safe_num_before_push = 4294967295,
    less_tpy_min_str = "-1",
    greater_typ_max_str = "4294967296",
    test_step_by = 8589934,
);

#[cfg(target_pointer_width = "64")]
define_unsigned_parser_and_test!(
    typ = usize,
    neg_typ = neg_usize,
    pos_typ = pos_usize,
    typ_str = "usize",
    repeat = 18,
    max_safe_num_before_push = 1844674407370955161,
    less_tpy_min_str = "-1",
    greater_typ_max_str = "18446744073709551616",
    test_step_by = 36170086419038336,
);
