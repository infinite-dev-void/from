use super::SyntaxErr;

const WHITESPACES: &[u8] = &[b'\t', b'\n', b'\r', b' '];

#[inline]
pub fn skip_whitespaces(json: &[u8], idx: &mut usize) {
    while json.len() > *idx {
        if WHITESPACES.contains(&json[*idx]) {
            *idx += 1;
            continue;
        }

        return;
    }
}

#[inline]
pub fn get_until_or_unexpected_end<'a>(
    json: &'a [u8],
    idx: &mut usize,
    stoppers: &[u8],
) -> Result<&'a [u8], SyntaxErr> {
    let start = *idx;

    while json.len() > *idx {
        if stoppers.contains(&json[*idx]) {
            return Ok(&json[start..*idx]);
        };
        *idx += 1;
    }

    Err(SyntaxErr::unexpected_end(idx))
}

#[inline]
pub fn get_or_unexpected_end(json: &[u8], idx: &mut usize) -> Result<u8, SyntaxErr> {
    if json.len() > *idx {
        return Ok(json[*idx]);
    };

    Err(SyntaxErr::unexpected_end(idx))
}

#[inline]
pub fn expect_and_skip(exp: u8, json: &[u8], idx: &mut usize) -> Result<(), SyntaxErr> {
    match json.get(*idx) {
        Some(b) => {
            if *b != exp {
                Err(SyntaxErr::unexpected_token(
                    unsafe { ::core::str::from_utf8_unchecked(&[exp]) },
                    &[*b],
                    idx,
                ))
            } else {
                *idx += 1;
                Ok(())
            }
        }
        None => Err(SyntaxErr::unexpected_end(idx)),
    }
}

#[inline]
pub fn skip_value(json: &[u8], idx: &mut usize) -> Result<(), SyntaxErr> {
    let byte = get_or_unexpected_end(json, idx)?;

    match byte {
        b'"' => skip_string(json, idx),
        b'{' => skip_object(json, idx),
        b'[' => skip_array(json, idx),
        b'0'..=b'9' => {
            skip_number(json, idx);
            Ok(())
        }
        b'n' => skip_null(json, idx, "null"),
        b't' => skip_true(json, idx, "true"),
        b'f' => skip_false(json, idx, "false"),
        _ => Err(SyntaxErr::unexpected_token(
            "['\"', '{', '[', '0..9']",
            &[byte],
            idx,
        )),
    }
}

#[inline]
pub fn skip_string(json: &[u8], idx: &mut usize) -> Result<(), SyntaxErr> {
    // idx refers to the opening quote of the string so one must be added
    *idx += 1;
    while json.len() > *idx {
        match json[*idx] {
            b'\\' => {
                // There is no need to check for \uxxxx because x must
                // not be double quotes and even if it is, the skip function
                // doesn't care, it just tries to skip the value.
                *idx += 2;
                continue;
            }

            b'"' => {
                *idx += 1;
                return Ok(());
            }

            _ => *idx += 1,
        }
    }

    Err(SyntaxErr::unexpected_end(idx))
}

#[inline]
// idx shoud be refer to the begining of the value -> t
pub fn skip_true(json: &[u8], idx: &mut usize, exp: &'static str) -> Result<(), SyntaxErr> {
    if json.len() < *idx + 4 {
        return Err(SyntaxErr::unexpected_end(&mut (json.len() + 1)));
    };

    if json[*idx + 1..=*idx + 3].eq("rue".as_bytes()) {
        // end of input
        if json.len() < *idx + 5 {
            *idx += 4;
            return Ok(());
        };

        match json[*idx + 4] {
            // possible ends
            b',' | b' ' | b'}' | b']' | b'\n' | b'\r' | b'\t' => {
                *idx += 4;
                return Ok(());
            }

            _ => {
                return Err(SyntaxErr::unexpected_token(exp, &[json[*idx]], idx));
            }
        };
    };

    Err(SyntaxErr::unexpected_token(exp, &[json[*idx]], idx))
}

#[inline]
// idx shoud be refer to the begining of the value -> f
pub fn skip_false(json: &[u8], idx: &mut usize, exp: &'static str) -> Result<(), SyntaxErr> {
    if json.len() < *idx + 5 {
        return Err(SyntaxErr::unexpected_end(&mut (json.len() + 1)));
    };

    if json[*idx + 1..=*idx + 4].eq("alse".as_bytes()) {
        // end of input
        if json.len() < *idx + 6 {
            *idx += 5;
            return Ok(());
        };

        match json[*idx + 5] {
            // possible ends
            b',' | b' ' | b'}' | b']' | b'\n' | b'\r' | b'\t' => {
                *idx += 5;
                return Ok(());
            }

            _ => {
                return Err(SyntaxErr::unexpected_token(exp, &[json[*idx]], idx));
            }
        };
    };

    Err(SyntaxErr::unexpected_token(exp, &[json[*idx]], idx))
}

#[inline]
pub fn skip_null(json: &[u8], idx: &mut usize, exp: &'static str) -> Result<(), SyntaxErr> {
    // idx refers to the begining of the value -> n
    if json.len() < *idx + 4 {
        return Err(SyntaxErr::unexpected_end(&mut (json.len() + 1)));
    };

    if json[*idx + 1..=*idx + 3].eq("ull".as_bytes()) {
        // end of input
        if json.len() < *idx + 5 {
            *idx += 4;
            return Ok(());
        };

        match json[*idx + 4] {
            // possible ends
            b',' | b' ' | b'}' | b']' | b'\n' | b'\r' | b'\t' => {
                *idx += 4;
                return Ok(());
            }

            _ => {
                return Err(SyntaxErr::unexpected_token(exp, &[json[*idx]], idx));
            }
        };
    };

    Err(SyntaxErr::unexpected_token(exp, &[json[*idx]], idx))
}

#[inline]
/// recursively skips nested arrays and strings within a JSON array, using a mutable index to keep track of the position.
pub fn skip_array(json: &[u8], idx: &mut usize) -> Result<(), SyntaxErr> {
    // idx refers to the opening brace of the array so one must be added
    *idx += 1;
    let mut depth = 0;
    while json.len() > *idx {
        match json[*idx] {
            b'[' => {
                *idx += 1;
                depth += 1;
            }

            b']' => {
                *idx += 1;
                if depth == 0 {
                    return Ok(());
                };
                depth -= 1;
            }

            b'"' => {
                *idx += 1;
                skip_string(json, idx)?;
            }

            _ => *idx += 1,
        }
    }

    Err(SyntaxErr::unexpected_end(idx))
}

#[inline]
/// recursively skips nested objects and strings within a JSON object, using a mutable index to keep track of the position.
pub fn skip_object(json: &[u8], idx: &mut usize) -> Result<(), SyntaxErr> {
    // idx refers to the opening bracket of the object so one must be added
    *idx += 1;
    let mut depth = 0;
    while json.len() > *idx {
        match json[*idx] {
            b'{' => {
                *idx += 1;
                depth += 1;
            }

            b'}' => {
                *idx += 1;
                if depth == 0 {
                    return Ok(());
                };
                depth -= 1;
            }

            b'"' => {
                *idx += 1;
                skip_string(json, idx)?;
            }

            _ => *idx += 1,
        }
    }

    Err(SyntaxErr::unexpected_end(idx))
}

#[inline]
pub fn skip_number(json: &[u8], idx: &mut usize) {
    *idx += 1;
    while json.len() > *idx {
        match json[*idx] {
            b',' | b' ' | b'}' | b']' | b'\n' | b'\r' | b'\t' => {
                return;
            }

            _ => *idx += 1,
        };
    }
}
