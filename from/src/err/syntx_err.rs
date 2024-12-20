#[derive(PartialEq, Eq, Debug)]
pub struct SyntaxErr {
    pub msg: String,

    pub offset: usize,
}

impl SyntaxErr {
    #[inline]
    pub fn new(msg: &str, offset: &mut usize) -> Self {
        Self {
            msg: String::from(msg),
            offset: *offset,
        }
    }

    #[inline]
    pub fn unexpected_token(exp: &str, found: &[u8], offset: &mut usize) -> Self {
        match core::str::from_utf8(found) {
            Ok(s) => {
                let mut msg = String::from("expected: ");
                msg.push_str(exp);
                msg.push_str(", found: ");
                msg.push_str(s);
                Self {
                    msg,
                    offset: *offset,
                }
            }

            Err(_) => Self::new("found an invalid utf8 byte", offset),
        }
    }

    #[inline]
    pub fn unexpected_end(offset: &mut usize) -> Self {
        Self {
            msg: String::from("unexpected end of json input"),
            offset: *offset,
        }
    }

    #[inline]
    pub fn to_json(&self) -> String {
        let mut json = String::from(r#"{"msg":"#);
        json.push_str(&super::string_to_json(&self.msg));
        json.push(',');
        json.push_str(r#""offset":"#);
        json.push_str(&self.offset.to_string());
        json.push('}');

        json
    }
}
