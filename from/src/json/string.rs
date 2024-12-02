use std::ops::Range;

use super::{utils, SyntaxErr};

#[derive(Debug, PartialEq)]
pub enum ParseResult {
    Ok(String),
    Null,
    TypeMismatch(String),
    SyntaxErr(SyntaxErr),
}

impl ParseResult {
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

const TWO_BYTES: Range<u8> = 0b11000000..0b11100000;
const THREE_BYTES: Range<u8> = 0b11100000..0b11110000;
const FOUR_BYTES: Range<u8> = 0b11110000..0b11111000;

const MINION_BYTE: Range<u8> = 0b10000000..0b11000000;

pub fn parse(json: &[u8], idx: &mut usize) -> ParseResult {
    let mut byte = match utils::get_or_unexpected_end(json, idx) {
        Ok(byte) => byte,
        Err(e) => return ParseResult::SyntaxErr(e),
    };

    match byte {
        b'"' => {}

        // null
        b'n' => {
            return if let Err(e) = utils::skip_null(json, idx, "\"") {
                ParseResult::SyntaxErr(e)
            } else {
                ParseResult::Null
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

        b'0'..=b'9' => {
            utils::skip_number(json, idx);
            return ParseResult::type_mismatch("number");
        }

        _ => {
            return ParseResult::unexpected_token("\"", byte, idx);
        }
    };

    let mut string = Vec::<u8>::new();

    macro_rules! get_next_byte {
        () => {
            *idx += 1;

            byte = match utils::get_or_unexpected_end(json, idx) {
                Ok(byte) => byte,
                Err(e) => return ParseResult::SyntaxErr(e),
            };
        };
    }

    let mut last_push = *idx + 1;

    // idx: 9, last_push: 10
    // idx: 10, last_push: 10
    // idx: 15, last_push: 10
    // idx: 15, last_push: 1
    loop {
        get_next_byte!();

        match byte {
            b'"' => {
                if last_push < *idx {
                    string.extend_from_slice(&json[last_push..*idx]);
                };
                *idx += 1;
                return ParseResult::Ok(unsafe { String::from_utf8_unchecked(string) });
            }

            b'\\' => {
                if last_push < *idx {
                    string.extend_from_slice(&json[last_push..*idx]);
                };
                get_next_byte!();

                match byte {
                    b'"' | b'\\' | b'/' => {
                        string.push(byte);
                        last_push = *idx + 1;
                    }

                    b'b' => {
                        string.push(8); // backspace
                        last_push = *idx + 1;
                    }

                    b'f' => {
                        string.push(12); // form feed
                        last_push = *idx + 1;
                    }

                    b't' => {
                        string.push(b'\t');
                        last_push = *idx + 1;
                    }

                    b'r' => {
                        string.push(b'\r');
                        last_push = *idx + 1;
                    }

                    b'n' => {
                        string.push(b'\n');
                        last_push = *idx + 1;
                    }

                    b'u' => {
                        // 4 hex + 1 required double quote
                        if json.len() < *idx + 5 {
                            return ParseResult::SyntaxErr(SyntaxErr::unexpected_end(
                                &mut json.len(),
                            ));
                        };

                        *idx += 1;

                        let mut hex;

                        byte = json[*idx];
                        match byte {
                            b'0'..=b'9' => hex = (byte - 48) as u16,

                            b'A'..=b'F' => hex = (byte - 55) as u16,

                            b'a'..=b'f' => hex = (byte - 87) as u16,

                            _ => return ParseResult::syntax_err("invalid hex digit", idx),
                        };

                        for _ in 0..3 {
                            *idx += 1;
                            byte = json[*idx];
                            hex *= 16;
                            match byte {
                                b'0'..=b'9' => hex += (byte - 48) as u16,

                                b'A'..=b'F' => hex += (byte - 55) as u16,

                                b'a'..=b'f' => hex += (byte - 87) as u16,

                                _ => return ParseResult::syntax_err("invalid hex digit", idx),
                            };
                        }

                        last_push = *idx + 1;

                        // 0XXXXXXX -> 0..=127
                        if hex < 128 {
                            string.push(hex as u8);
                            continue;
                        };

                        // 110XXXXX  10XX XXXX  -> 128..=2047
                        if hex < 2048 {
                            string.extend_from_slice(&[
                                (hex >> 6) as u8 | 0b1100_0000,
                                (hex as u8 & 0b0011_1111) | 0b1000_0000,
                            ]);

                            continue;
                        };

                        // 1110XXXX  10XX XXXX  10XX XXXX  -> 2048..=65535

                        // take first four bits to the right and set the
                        // remaining four bits at left to 1110
                        string.extend_from_slice(&[
                            (hex >> 12) as u8 | 0b1110_0000,
                            ((hex >> 6) as u8 & 0b0011_1111) | 0b1000_0000,
                            (hex as u8 & 0b0011_1111) | 0b1000_0000,
                        ]);
                    }

                    _ => return ParseResult::syntax_err("invalid control character", idx),
                }
            }

            _ => {
                // Byte -> 0XXX XXXX
                if byte < 128 {
                    continue;
                };

                //
                //
                //
                // 2 Bytes 110X XXXX  10XX XXXX
                if TWO_BYTES.contains(&byte) {
                    get_next_byte!();

                    if !MINION_BYTE.contains(&byte) {
                        return ParseResult::syntax_err("invalid UTF-8 byte", idx);
                    };
                    continue;
                };

                //
                //
                //
                //
                //
                // 3 Bytes 1110 XXXX  10XX XXXXX  10XX XXXX
                if THREE_BYTES.contains(&byte) {
                    for _ in 0..2 {
                        get_next_byte!();

                        if !MINION_BYTE.contains(&byte) {
                            return ParseResult::syntax_err("invalid UTF-8 byte", idx);
                        };
                    }

                    continue;
                }

                //
                //
                //
                //
                // 4 Bytes 1111 0XXX  10XX XXXX  10XX XXXX  10XX XXXX
                if FOUR_BYTES.contains(&byte) {
                    for _ in 0..3 {
                        get_next_byte!();

                        if !MINION_BYTE.contains(&byte) {
                            return ParseResult::syntax_err("invalid UTF-8 byte", idx);
                        };
                    }

                    continue;
                };

                //
                //
                // invalid must not be outside the previous ranges
                return ParseResult::syntax_err("invalid UTF-8 byte", idx);
            }
        }
    }
}

pub fn sanitize_xss(s: &mut String) {
    const QUOT: &[u8] = b"&#34;";
    const APOS: &[u8] = b"&#39;";
    const AMP: &[u8] = b"&amp;";
    const LT: &[u8] = b"&lt;";
    const GT: &[u8] = b"&gt;";
    const NULL: &[u8] = "\u{FFFD}".as_bytes();

    let bytes = s.as_bytes();
    let mut es: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut last = 0usize;
    let mut idx = 0usize;

    while bytes.len() > idx {
        match bytes[idx] {
            b'\0' => {
                if last < idx {
                    es.extend_from_slice(&bytes[last..idx]);
                };
                es.extend_from_slice(NULL);
                last = idx + 1;
            }
            b'"' => {
                if last < idx {
                    es.extend_from_slice(&bytes[last..idx]);
                };
                es.extend_from_slice(QUOT);
                last = idx + 1;
            }
            b'\'' => {
                if last < idx {
                    es.extend_from_slice(&bytes[last..idx]);
                };
                es.extend_from_slice(APOS);
                last = idx + 1;
            }
            b'&' => {
                if last < idx {
                    es.extend_from_slice(&bytes[last..idx]);
                };
                es.extend_from_slice(AMP);
                last = idx + 1;
            }
            b'<' => {
                if last < idx {
                    es.extend_from_slice(&bytes[last..idx]);
                };
                es.extend_from_slice(LT);
                last = idx + 1;
            }
            b'>' => {
                if last < idx {
                    es.extend_from_slice(&bytes[last..idx]);
                };
                es.extend_from_slice(GT);
                last = idx + 1;
            }

            _ => {}
        }
        idx += 1;
    }

    if last < s.len() {
        es.extend_from_slice(&bytes[last..]);
    };

    *s = unsafe { String::from_utf8_unchecked(es) }
}

#[cfg(test)]
mod test {

    use crate::json::SyntaxErr;

    use super::{parse, sanitize_xss, ParseResult};

    #[test]
    fn valid() {
        assert_eq!(
            parse(r#""hello\n\t\u00c2\b\f\uf977\r\"""#.as_bytes(), &mut 0),
            ParseResult::Ok(String::from("hello\n\tÂ亮\r\""))
        );
    }

    #[test]
    fn null() {
        assert_eq!(parse("null".as_bytes(), &mut 0), ParseResult::Null);
    }

    #[test]
    fn type_mismatch_false() {
        assert_eq!(
            parse("false,".as_bytes(), &mut 0),
            ParseResult::type_mismatch("boolean")
        );
    }

    #[test]
    fn type_mismatch_true() {
        assert_eq!(
            parse("true}".as_bytes(), &mut 0),
            ParseResult::type_mismatch("boolean")
        );
    }

    #[test]
    fn type_mismatch_object() {
        assert_eq!(
            parse("{}".as_bytes(), &mut 0),
            ParseResult::type_mismatch("object")
        );
    }

    #[test]
    fn type_mismatch_array() {
        assert_eq!(
            parse("[]".as_bytes(), &mut 0),
            ParseResult::type_mismatch("array")
        );
    }

    #[test]
    fn type_mismatch_number() {
        // if the first character is a decimal digit then the
        // parser assums that the value is of type number
        // and will skip the following bytes without validate
        // them until it found a possible end:
        // [",", "}", "]", " ", "\t", "\n", end_of_input]
        assert_eq!(
            parse("4sadqwd.asdqwe".as_bytes(), &mut 0),
            ParseResult::type_mismatch("number")
        );
    }

    #[test]
    fn syntax_err() {
        assert_eq!(
            parse("fcvsd".as_bytes(), &mut 0),
            ParseResult::SyntaxErr(SyntaxErr::unexpected_token("\"", "f".as_bytes(), &mut 0))
        );

        assert_eq!(
            parse("nula".as_bytes(), &mut 0),
            ParseResult::SyntaxErr(SyntaxErr::unexpected_token("\"", "n".as_bytes(), &mut 0))
        );

        assert_eq!(
            parse("truee".as_bytes(), &mut 0),
            ParseResult::SyntaxErr(SyntaxErr::unexpected_token("\"", "t".as_bytes(), &mut 0))
        );

        assert_eq!(
            parse("asda".as_bytes(), &mut 0),
            ParseResult::SyntaxErr(SyntaxErr::unexpected_token("\"", "a".as_bytes(), &mut 0))
        );

        assert_eq!(
            parse(r#""aswq"#.as_bytes(), &mut 0),
            ParseResult::SyntaxErr(SyntaxErr::unexpected_end(&mut 5))
        );
    }

    #[test]
    fn invalid_utf8_byte() {
        // minion byte must be greater than 127 not 17
        assert_eq!(
            parse(&[b'"', 192, 17, b'"'], &mut 0),
            ParseResult::syntax_err("invalid UTF-8 byte", &mut 2)
        );

        // leading byte must be covered by one of valid ranges
        assert_eq!(
            parse(&[b'"', 182, b'"'], &mut 0),
            ParseResult::syntax_err("invalid UTF-8 byte", &mut 1)
        );

        // must have extra byte before "
        assert_eq!(
            parse(&[b'"', 192, b'"'], &mut 0),
            ParseResult::syntax_err("invalid UTF-8 byte", &mut 2)
        );

        // must have extra byte before "
        assert_eq!(
            parse(&[b'"', 244, 128, b'"'], &mut 0),
            ParseResult::syntax_err("invalid UTF-8 byte", &mut 3)
        );
    }

    #[test]
    fn invalid_control_character() {
        assert_eq!(
            parse(r#""\h""#.as_bytes(), &mut 0),
            ParseResult::syntax_err("invalid control character", &mut 2)
        );
    }

    #[test]
    fn xss_sanitize() {
        let mut s = String::from("<h1>hello & nice \" ' \u{0000}</h1>");
        sanitize_xss(&mut s);
        assert_eq!(
            s,
            String::from("&lt;h1&gt;hello &amp; nice &#34; &#39; \u{FFFD}&lt;/h1&gt;")
        );
    }
}
