pub enum CheckResult {
    Ok,
    Null,
    TypeMismatch(&'static str),
    SyntaxErr(super::SyntaxErr),
}

use super::utils;

#[inline]
pub fn check(json: &[u8], idx: &mut usize) -> CheckResult {
    let byte = match utils::get_or_unexpected_end(json, idx) {
        Ok(byte) => byte,
        Err(e) => return CheckResult::SyntaxErr(e),
    };

    match byte {
        b'{' => {
            *idx += 1;
            CheckResult::Ok
        }

        b'"' => {
            *idx += 1;
            if let Err(e) = utils::skip_string(json, idx) {
                CheckResult::SyntaxErr(e)
            } else {
                CheckResult::TypeMismatch("string")
            }
        }

        // null
        b'n' => {
            if let Err(e) = utils::skip_null(json, idx, "[") {
                CheckResult::SyntaxErr(e)
            } else {
                CheckResult::Null
            }
        }

        b'f' => {
            if let Err(e) = utils::skip_false(json, idx, "[") {
                CheckResult::SyntaxErr(e)
            } else {
                CheckResult::TypeMismatch("boolean")
            }
        }

        b't' => {
            return if let Err(e) = utils::skip_true(json, idx, "[") {
                CheckResult::SyntaxErr(e)
            } else {
                CheckResult::TypeMismatch("boolean")
            }
        }

        b'[' => {
            return if let Err(e) = utils::skip_array(json, idx) {
                CheckResult::SyntaxErr(e)
            } else {
                CheckResult::TypeMismatch("array")
            }
        }

        b'0'..=b'9' => {
            utils::skip_number(json, idx);
            CheckResult::TypeMismatch("number")
        }

        _ => CheckResult::SyntaxErr(super::SyntaxErr::unexpected_token("[", &[byte], idx)),
    }
}
