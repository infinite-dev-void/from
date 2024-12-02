use super::{utils, SyntaxErr};

#[derive(Debug, PartialEq)]
pub enum ParseResult {
    Ok(bool),
    Null,
    TypeMismatch(String),
    SyntaxErr(SyntaxErr),
}

impl ParseResult {
    /* #[inline]
    fn syntax_err(msg: &str, idx: &mut usize) -> Self {
        Self::SyntaxErr(SyntaxErr::new(msg, idx))
    } */

    #[inline]
    fn unexpected_token(exp: &str, found: u8, idx: &mut usize) -> Self {
        Self::SyntaxErr(SyntaxErr::unexpected_token(exp, &[found], idx))
    }

    #[inline]
    fn type_mismatch(found: &str) -> Self {
        ParseResult::TypeMismatch(String::from(found))
    }
}

pub fn parse(json: &[u8], idx: &mut usize) -> ParseResult {
    let byte = match utils::get_or_unexpected_end(json, idx) {
        Ok(byte) => byte,
        Err(e) => return ParseResult::SyntaxErr(e),
    };

    match byte {
        b'f' => {
            if let Err(e) = utils::skip_false(json, idx, "boolean") {
                ParseResult::SyntaxErr(e)
            } else {
                ParseResult::Ok(false)
            }
        }

        b't' => {
            if let Err(e) = utils::skip_true(json, idx, "boolean") {
                ParseResult::SyntaxErr(e)
            } else {
                ParseResult::Ok(true)
            }
        }

        // null
        b'n' => {
            if let Err(e) = utils::skip_null(json, idx, "boolean") {
                ParseResult::SyntaxErr(e)
            } else {
                ParseResult::Null
            }
        }

        b'"' => {
            if let Err(e) = utils::skip_string(json, idx) {
                ParseResult::SyntaxErr(e)
            } else {
                ParseResult::type_mismatch("string")
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

        b'0'..=b'9' => {
            utils::skip_number(json, idx);
            ParseResult::type_mismatch("number")
        }

        _ => ParseResult::unexpected_token("\"", byte, idx),
    }
}

#[cfg(test)]
mod test {

    use super::ParseResult;

    #[test]
    fn valid() {
        assert_eq!(
            super::parse("false,".as_bytes(), &mut 0),
            ParseResult::Ok(false),
        );

        assert_eq!(
            super::parse("true".as_bytes(), &mut 0),
            ParseResult::Ok(true),
        );
    }

    #[test]
    fn null() {
        assert_eq!(super::parse("null ".as_bytes(), &mut 0), ParseResult::Null,);
    }

    #[test]
    fn type_mismatch_string() {
        assert_eq!(
            super::parse(r#""""#.as_bytes(), &mut 0),
            ParseResult::type_mismatch("string"),
        );
    }

    #[test]
    fn type_mismatch_number() {
        assert_eq!(
            super::parse(r#"1as"#.as_bytes(), &mut 0),
            ParseResult::type_mismatch("number"),
        );
    }

    #[test]
    fn type_mismatch_object() {
        assert_eq!(
            super::parse(r#"{asdasd{}a}"#.as_bytes(), &mut 0),
            ParseResult::type_mismatch("object"),
        );
    }

    #[test]
    fn type_mismatch_array() {
        assert_eq!(
            super::parse(r#"[asda[]aasda[asss]]"#.as_bytes(), &mut 0),
            ParseResult::type_mismatch("array"),
        );
    }

    #[test]
    fn syntax_err() {
        assert_eq!(
            super::parse("falsewq".as_bytes(), &mut 0),
            ParseResult::unexpected_token("boolean", b'f', &mut 0),
        );

        assert_eq!(
            super::parse("ttrue".as_bytes(), &mut 0),
            ParseResult::unexpected_token("boolean", b't', &mut 0),
        );

        assert_eq!(
            super::parse("null_".as_bytes(), &mut 0),
            ParseResult::unexpected_token("boolean", b'n', &mut 0),
        );
    }
}
