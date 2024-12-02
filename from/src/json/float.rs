use std::str::FromStr;

use super::{utils, SyntaxErr};

#[derive(Debug, PartialEq)]
pub enum ParseResult<T: FromStr> {
    Ok(T),
    Null,
    TypeMismatch(String),
    SyntaxErr(SyntaxErr),
}

impl<T: FromStr> ParseResult<T> {
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
}

#[inline]
fn parse_float<T: FromStr>(bytes: &[u8], mut idx: usize) -> ParseResult<T> {
    unsafe {
        match ::core::str::from_utf8_unchecked(bytes).parse() {
            Ok(f) => ParseResult::Ok(f),
            Err(_) => ParseResult::syntax_err("invalid float literal", &mut idx),
        }
    }
}

pub mod f32 {
    use super::{parse_float, utils};

    pub type ParseResult = super::ParseResult<f32>;

    pub fn parse(json: &[u8], idx: &mut usize) -> ParseResult {
        let mut byte = match utils::get_or_unexpected_end(json, idx) {
            Ok(byte) => byte,
            Err(e) => return ParseResult::SyntaxErr(e),
        };

        let begin = *idx;

        macro_rules! must_get_byte {
            () => {
                byte = match utils::get_or_unexpected_end(json, idx) {
                    Ok(byte) => byte,
                    Err(e) => return ParseResult::SyntaxErr(e),
                };
            };
        }

        macro_rules! must_get_next_byte {
            () => {
                *idx += 1;
                must_get_byte!();
            };
        }

        match byte {
            b'0'..=b'9' => {}

            b'-' => {
                must_get_next_byte!();

                if !(b'0'..=b'9').contains(&byte) {
                    return ParseResult::unexpected_token("digit", byte, idx);
                };
            }

            b'n' => {
                return if let Err(e) = utils::skip_null(json, idx, "\"") {
                    ParseResult::SyntaxErr(e)
                } else {
                    ParseResult::Null
                }
            }

            b'"' => {
                return if let Err(e) = utils::skip_string(json, idx) {
                    ParseResult::SyntaxErr(e)
                } else {
                    ParseResult::type_mismatch("string")
                }
            }

            b'f' => {
                return if let Err(e) = utils::skip_false(json, idx, "\"") {
                    ParseResult::SyntaxErr(e)
                } else {
                    ParseResult::type_mismatch("boolean")
                }
            }

            b't' => {
                return if let Err(e) = utils::skip_true(json, idx, "\"") {
                    ParseResult::SyntaxErr(e)
                } else {
                    ParseResult::type_mismatch("boolean")
                }
            }

            b'{' => {
                return if let Err(e) = utils::skip_object(json, idx) {
                    ParseResult::SyntaxErr(e)
                } else {
                    ParseResult::type_mismatch("object")
                }
            }

            b'[' => {
                return if let Err(e) = utils::skip_array(json, idx) {
                    ParseResult::SyntaxErr(e)
                } else {
                    ParseResult::type_mismatch("array")
                }
            }
            _ => {
                return ParseResult::unexpected_token("digit", byte, idx);
            }
        };

        macro_rules! get_byte_or_return {
            () => {
                byte = match json.get(*idx) {
                    Some(byte) => *byte,
                    None => return parse_float::<f32>(&json[begin..*idx], begin),
                };
            };
        }

        macro_rules! byte_must_be_digit {
            () => {
                if !(b'0'..=b'9').contains(&byte) {
                    return ParseResult::unexpected_token("digit", byte, idx);
                };
            };
        }

        let mut dot = false;

        loop {
            *idx += 1;

            get_byte_or_return!();

            match byte {
                b'0'..=b'9' => {}

                b',' | b' ' | b'}' | b']' | b'\n' | b'\r' | b'\t' => {
                    return parse_float::<f32>(&json[begin..*idx], begin)
                }

                b'.' => {
                    if dot {
                        return ParseResult::unexpected_token("digit", byte, idx);
                    };

                    dot = true;
                    must_get_next_byte!();
                    byte_must_be_digit!();
                }

                b'e' | b'E' => {
                    must_get_next_byte!();

                    match byte {
                        b'0'..=b'9' => {}
                        b'-' | b'+' => {
                            must_get_next_byte!();
                            byte_must_be_digit!();
                        }

                        _ => {
                            return ParseResult::unexpected_token("[digit, -, +]", byte, idx);
                        }
                    };

                    loop {
                        *idx += 1;
                        get_byte_or_return!();

                        match byte {
                            b'0'..=b'9' => {}

                            b',' | b' ' | b'}' | b']' | b'\n' | b'\r' | b'\t' => {
                                return parse_float::<f32>(&json[begin..*idx], begin)
                            }

                            _ => return ParseResult::unexpected_token("digit", byte, idx),
                        }
                    }
                }

                _ => return ParseResult::unexpected_token("digit", byte, idx),
            };
        }
    }

    #[cfg(test)]
    mod test {
        use core::f32;

        use crate::json::SyntaxErr;

        use super::ParseResult;
        #[test]
        fn valid() {
            assert_eq!(
                super::parse("18.4879e2,".as_bytes(), &mut 0),
                ParseResult::Ok(1848.79),
            );

            assert_eq!(
                super::parse("875.497}".as_bytes(), &mut 0),
                ParseResult::Ok(875.497),
            );

            assert_eq!(
                super::parse("875.497e-4]".as_bytes(), &mut 0),
                ParseResult::Ok(0.0875497),
            );

            assert_eq!(
                super::parse("0.497]".as_bytes(), &mut 0),
                ParseResult::Ok(0.497),
            );

            assert_eq!(
                super::parse("-0.497]".as_bytes(), &mut 0),
                ParseResult::Ok(-0.497),
            );

            assert_eq!(
                super::parse("-47}".as_bytes(), &mut 0),
                ParseResult::Ok(-47f32),
            );

            assert_eq!(
                super::parse("04 ".as_bytes(), &mut 0),
                ParseResult::Ok(4f32),
            );

            assert_eq!(
                super::parse("-0".as_bytes(), &mut 0),
                ParseResult::Ok(-0f32),
            );

            assert_eq!(
                super::parse("3.4028235e38".as_bytes(), &mut 0),
                ParseResult::Ok(f32::MAX),
            );

            assert_eq!(
                super::parse("-3.4028235e38".as_bytes(), &mut 0),
                ParseResult::Ok(f32::MIN),
            );
        }

        #[test]
        fn infinite() {
            assert_eq!(
                super::parse("3.41e38".as_bytes(), &mut 0),
                ParseResult::Ok(f32::INFINITY),
            );

            assert_eq!(
                super::parse("-3.41e38".as_bytes(), &mut 0),
                ParseResult::Ok(f32::NEG_INFINITY),
            );
        }

        #[test]
        fn null() {
            assert_eq!(super::parse("null\t".as_bytes(), &mut 0), ParseResult::Null,)
        }

        #[test]
        fn type_mismatch_bool() {
            assert_eq!(
                super::parse("false\n".as_bytes(), &mut 0),
                ParseResult::type_mismatch("boolean")
            );

            assert_eq!(
                super::parse("true\n".as_bytes(), &mut 0),
                ParseResult::type_mismatch("boolean")
            );
        }

        #[test]
        fn type_mismatch_string() {
            assert_eq!(
                super::parse(r#""""#.as_bytes(), &mut 0),
                ParseResult::type_mismatch("string"),
            );
        }

        #[test]

        fn type_mismatch_object() {
            assert_eq!(
                super::parse(r#"{asdasdq47897qwsa.as!@#!@#}"#.as_bytes(), &mut 0),
                ParseResult::type_mismatch("object"),
            );
        }

        #[test]

        fn type_mismatch_array() {
            assert_eq!(
                super::parse(r#"[{dqw2412oczxc.789/*-}]"#.as_bytes(), &mut 0),
                ParseResult::type_mismatch("array"),
            );
        }

        #[test]
        fn syntax_err() {
            assert_eq!(
                super::parse("74..3".as_bytes(), &mut 0),
                ParseResult::unexpected_token("digit", b'.', &mut 3)
            );

            assert_eq!(
                super::parse("74.".as_bytes(), &mut 0),
                ParseResult::SyntaxErr(SyntaxErr::unexpected_end(&mut 3))
            );

            assert_eq!(
                super::parse("74e".as_bytes(), &mut 0),
                ParseResult::SyntaxErr(SyntaxErr::unexpected_end(&mut 3))
            );

            assert_eq!(
                super::parse("74e-".as_bytes(), &mut 0),
                ParseResult::SyntaxErr(SyntaxErr::unexpected_end(&mut 4))
            );

            assert_eq!(
                super::parse("-".as_bytes(), &mut 0),
                ParseResult::SyntaxErr(SyntaxErr::unexpected_end(&mut 1))
            );

            assert_eq!(
                super::parse("".as_bytes(), &mut 0),
                ParseResult::SyntaxErr(SyntaxErr::unexpected_end(&mut 0))
            );

            assert_eq!(
                super::parse("1ee".as_bytes(), &mut 0),
                ParseResult::unexpected_token("[digit, -, +]", b'e', &mut 2)
            );
        }
    }
}

pub mod f64 {
    use super::{parse_float, utils};

    pub type ParseResult = super::ParseResult<f64>;

    pub fn parse(json: &[u8], idx: &mut usize) -> ParseResult {
        let mut byte = match utils::get_or_unexpected_end(json, idx) {
            Ok(byte) => byte,
            Err(e) => return ParseResult::SyntaxErr(e),
        };

        let begin = *idx;

        macro_rules! must_get_byte {
            () => {
                byte = match utils::get_or_unexpected_end(json, idx) {
                    Ok(byte) => byte,
                    Err(e) => return ParseResult::SyntaxErr(e),
                };
            };
        }

        macro_rules! must_get_next_byte {
            () => {
                *idx += 1;
                must_get_byte!();
            };
        }

        match byte {
            b'0'..=b'9' => {}

            b'-' => {
                must_get_next_byte!();

                if !(b'0'..=b'9').contains(&byte) {
                    return ParseResult::unexpected_token("digit", byte, idx);
                };
            }

            b'n' => {
                return if let Err(e) = utils::skip_null(json, idx, "\"") {
                    ParseResult::SyntaxErr(e)
                } else {
                    ParseResult::Null
                }
            }

            b'"' => {
                return if let Err(e) = utils::skip_string(json, idx) {
                    ParseResult::SyntaxErr(e)
                } else {
                    ParseResult::type_mismatch("string")
                }
            }

            b'f' => {
                return if let Err(e) = utils::skip_false(json, idx, "\"") {
                    ParseResult::SyntaxErr(e)
                } else {
                    ParseResult::type_mismatch("boolean")
                }
            }

            b't' => {
                return if let Err(e) = utils::skip_true(json, idx, "\"") {
                    ParseResult::SyntaxErr(e)
                } else {
                    ParseResult::type_mismatch("boolean")
                }
            }

            b'{' => {
                return if let Err(e) = utils::skip_object(json, idx) {
                    ParseResult::SyntaxErr(e)
                } else {
                    ParseResult::type_mismatch("object")
                }
            }

            b'[' => {
                return if let Err(e) = utils::skip_array(json, idx) {
                    ParseResult::SyntaxErr(e)
                } else {
                    ParseResult::type_mismatch("array")
                }
            }
            _ => {
                return ParseResult::unexpected_token("digit", byte, idx);
            }
        };

        macro_rules! get_byte_or_return {
            () => {
                byte = match json.get(*idx) {
                    Some(byte) => *byte,
                    None => return parse_float::<f64>(&json[begin..*idx], begin),
                };
            };
        }

        macro_rules! byte_must_be_digit {
            () => {
                if !(b'0'..=b'9').contains(&byte) {
                    return ParseResult::unexpected_token("digit", byte, idx);
                };
            };
        }

        let mut dot = false;

        loop {
            *idx += 1;

            get_byte_or_return!();

            match byte {
                b'0'..=b'9' => {}

                b',' | b' ' | b'}' | b']' | b'\n' | b'\r' | b'\t' => {
                    return parse_float::<f64>(&json[begin..*idx], begin)
                }

                b'.' => {
                    if dot {
                        return ParseResult::unexpected_token("digit", byte, idx);
                    };

                    dot = true;
                    must_get_next_byte!();
                    byte_must_be_digit!();
                }

                b'e' | b'E' => {
                    must_get_next_byte!();

                    match byte {
                        b'0'..=b'9' => {}
                        b'-' | b'+' => {
                            must_get_next_byte!();
                            byte_must_be_digit!();
                        }

                        _ => {
                            return ParseResult::unexpected_token("[digit, -, +]", byte, idx);
                        }
                    };

                    loop {
                        *idx += 1;
                        get_byte_or_return!();

                        match byte {
                            b'0'..=b'9' => {}

                            b',' | b' ' | b'}' | b']' | b'\n' | b'\r' | b'\t' => {
                                return parse_float::<f64>(&json[begin..*idx], begin)
                            }

                            _ => return ParseResult::unexpected_token("digit", byte, idx),
                        }
                    }
                }

                _ => return ParseResult::unexpected_token("digit", byte, idx),
            };
        }
    }

    #[cfg(test)]
    mod test {
        use crate::json::SyntaxErr;

        use super::ParseResult;
        #[test]
        fn valid() {
            assert_eq!(
                super::parse("18.4879e2,".as_bytes(), &mut 0),
                ParseResult::Ok(1848.79),
            );

            assert_eq!(
                super::parse("875.497}".as_bytes(), &mut 0),
                ParseResult::Ok(875.497),
            );

            assert_eq!(
                super::parse("875.497e-4]".as_bytes(), &mut 0),
                ParseResult::Ok(0.0875497),
            );

            assert_eq!(
                super::parse("0.497]".as_bytes(), &mut 0),
                ParseResult::Ok(0.497),
            );

            assert_eq!(
                super::parse("-0.497]".as_bytes(), &mut 0),
                ParseResult::Ok(-0.497),
            );

            assert_eq!(
                super::parse("-47}".as_bytes(), &mut 0),
                ParseResult::Ok(-47f64),
            );

            assert_eq!(
                super::parse("04 ".as_bytes(), &mut 0),
                ParseResult::Ok(4f64),
            );

            assert_eq!(
                super::parse("-0".as_bytes(), &mut 0),
                ParseResult::Ok(-0f64),
            );

            // max
            assert_eq!(
                super::parse("1.7976931348623157e308".as_bytes(), &mut 0),
                ParseResult::Ok(f64::MAX),
            );

            // min
            assert_eq!(
                super::parse("-1.7976931348623157e308".as_bytes(), &mut 0),
                ParseResult::Ok(f64::MIN),
            );
        }

        #[test]
        fn infinite() {
            assert_eq!(
                super::parse("1.798e308".as_bytes(), &mut 0),
                ParseResult::Ok(f64::INFINITY),
            );

            assert_eq!(
                super::parse("-1.798e308".as_bytes(), &mut 0),
                ParseResult::Ok(f64::NEG_INFINITY),
            );
        }

        #[test]
        fn null() {
            assert_eq!(super::parse("null\t".as_bytes(), &mut 0), ParseResult::Null,)
        }

        #[test]
        fn type_mismatch_bool() {
            assert_eq!(
                super::parse("false\n".as_bytes(), &mut 0),
                ParseResult::type_mismatch("boolean")
            );

            assert_eq!(
                super::parse("true\n".as_bytes(), &mut 0),
                ParseResult::type_mismatch("boolean")
            );
        }

        #[test]
        fn type_mismatch_string() {
            assert_eq!(
                super::parse(r#""""#.as_bytes(), &mut 0),
                ParseResult::type_mismatch("string"),
            );
        }

        #[test]

        fn type_mismatch_object() {
            assert_eq!(
                super::parse(r#"{asdasdq47897qwsa.as!@#!@#}"#.as_bytes(), &mut 0),
                ParseResult::type_mismatch("object"),
            );
        }

        #[test]

        fn type_mismatch_array() {
            assert_eq!(
                super::parse(r#"[{dqw2412oczxc.789/*-}]"#.as_bytes(), &mut 0),
                ParseResult::type_mismatch("array"),
            );
        }

        #[test]
        fn syntax_err() {
            assert_eq!(
                super::parse("74..3".as_bytes(), &mut 0),
                ParseResult::unexpected_token("digit", b'.', &mut 3)
            );

            assert_eq!(
                super::parse("74.".as_bytes(), &mut 0),
                ParseResult::SyntaxErr(SyntaxErr::unexpected_end(&mut 3))
            );

            assert_eq!(
                super::parse("74e".as_bytes(), &mut 0),
                ParseResult::SyntaxErr(SyntaxErr::unexpected_end(&mut 3))
            );

            assert_eq!(
                super::parse("74e-".as_bytes(), &mut 0),
                ParseResult::SyntaxErr(SyntaxErr::unexpected_end(&mut 4))
            );

            assert_eq!(
                super::parse("-".as_bytes(), &mut 0),
                ParseResult::SyntaxErr(SyntaxErr::unexpected_end(&mut 1))
            );

            assert_eq!(
                super::parse("".as_bytes(), &mut 0),
                ParseResult::SyntaxErr(SyntaxErr::unexpected_end(&mut 0))
            );

            assert_eq!(
                super::parse("1ee".as_bytes(), &mut 0),
                ParseResult::unexpected_token("[digit, -, +]", b'e', &mut 2)
            );
        }
    }
}
