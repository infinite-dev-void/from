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

    #[inline]
    pub fn to_json(&self) -> String {
        match self {
            Self::SyntaxErr(s) => s.to_json(),
            Self::ValidationErr(v) => v.to_json(),
        }
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

    #[inline]
    pub fn to_json(&self) -> String {
        match self {
            Self::SyntaxErr(s) => s.to_json(),
            Self::ValidationErrs(vs) => {
                if vs.len() == 0 {
                    return String::from("[]");
                }
                let mut json = String::from('[');
                let mut i = 0;

                while vs.len() - 1 > i {
                    json.push_str(&vs[i].to_json());
                    json.push(',');
                    i += 1;
                }

                json.push_str(&vs[i].to_json());

                json.push(']');

                json
            }
        }
    }
}

mod test {
    use super::{Err, Errs, SyntaxErr, ValidationErr};

    #[test]
    fn err_sytax_err_to_json() {
        assert_eq!(
            Err::SyntaxErr(SyntaxErr::new("test\" \n test\\", &mut 5)).to_json(),
            String::from("{\"msg\":\"test\\\" \n test\\\\\",\"offset\":5}"),
        )
    }

    #[test]
    fn err_validation_err_to_json() {
        assert_eq!(
            Err::new_validation_err(
                From::from("field"),
                vec![From::from("obj"), From::from(1)],
                "test\""
            )
            .to_json(),
            String::from("{\"target\":\"field\",\"path\":[\"obj\",1],\"msg\":\"test\\\"\"}")
        )
    }

    #[test]
    fn errs_sytax_err_to_json() {
        assert_eq!(
            Errs::SyntaxErr(SyntaxErr::new("test\" \n test\\", &mut 5)).to_json(),
            String::from("{\"msg\":\"test\\\" \n test\\\\\",\"offset\":5}"),
        )
    }

    #[test]
    fn errs_validation_err_to_json() {
        assert_eq!(
            Errs::ValidationErrs(vec![
                ValidationErr::new(
                    From::from("field"),
                    vec![From::from("obj"), From::from(1)],
                    "test\""
                ),

                ValidationErr::new(
                    From::from(5),
                    vec![From::from(7), From::from("field"),],
                    "\"nice"
                )

            ])
            .to_json(),
            String::from("[{\"target\":\"field\",\"path\":[\"obj\",1],\"msg\":\"test\\\"\"},{\"target\":5,\"path\":[7,\"field\"],\"msg\":\"\\\"nice\"}]")
        )
    }
}
