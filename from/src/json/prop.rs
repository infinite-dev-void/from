use super::{utils, SyntaxErr};

pub fn parse<'a>(json: &'a [u8], idx: &mut usize) -> Result<&'a [u8], SyntaxErr> {
    let mut byte = utils::get_or_unexpected_end(json, idx)?;

    if byte.ne(&b'"') {
        return Err(SyntaxErr::unexpected_token("\"", &[byte], idx));
    };

    *idx += 1;

    let begin = *idx;

    byte = utils::get_or_unexpected_end(json, idx)?;

    match byte {
        b'a'..=b'z' | b'A'..=b'Z' | b'_' => {}

        _ => {
            return Err(SyntaxErr::unexpected_token(
                "alphabet character or underscore",
                &[byte],
                idx,
            ));
        }
    };

    loop {
        *idx += 1;

        byte = utils::get_or_unexpected_end(json, idx)?;

        match byte {
            b'a'..=b'z' | b'A'..=b'Z' | b'_' | b'0'..=b'9' => {}

            b'"' => {
                *idx += 1;
                return Ok(&json[begin..*idx - 1]);
            }

            _ => {
                return Err(SyntaxErr::unexpected_token(
                    "alphabet character, underscore or digit",
                    &[byte],
                    idx,
                ));
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{parse, SyntaxErr};
    #[test]
    fn valid() {
        assert_eq!(
            parse(r#""prop""#.as_bytes(), &mut 0),
            Ok(b"prop".as_slice()),
        );

        assert_eq!(
            parse(r#""_prop""#.as_bytes(), &mut 0),
            Ok(b"_prop".as_slice()),
        );

        assert_eq!(
            parse(r#""_12prop45""#.as_bytes(), &mut 0),
            Ok(b"_12prop45".as_slice()),
        );

        assert_eq!(
            parse(r#""_12prop45""#.as_bytes(), &mut 0),
            Ok(b"_12prop45".as_slice()),
        );
    }

    #[test]
    fn syntax_err() {
        assert_eq!(
            parse(r#"as"#.as_bytes(), &mut 0),
            Err(SyntaxErr::new("expected: \", found: a", &mut 0)),
        );

        assert_eq!(
            parse(r#""17asf""#.as_bytes(), &mut 0),
            Err(SyntaxErr::new(
                "expected: alphabet character or underscore, found: 1",
                &mut 1
            )),
        );

        assert_eq!(
            parse(r#""abcd"#.as_bytes(), &mut 0),
            Err(SyntaxErr::unexpected_end(&mut 5)),
        );

        assert_eq!(
            parse(r#""abcd#""#.as_bytes(), &mut 0),
            Err(SyntaxErr::unexpected_token(
                "alphabet character, underscore or digit",
                &[b'#'],
                &mut 5
            )),
        );
    }
}
