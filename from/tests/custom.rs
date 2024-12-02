use from::{from, FromJson, Null, OptionNull, Path, ValidationErr};

#[derive(Debug, PartialEq)]
#[from(json)]
struct Custom {
    #[max_len(value = 2)]
    sub_field: Vec<Null<u8>>,
}

impl Default for Custom {
    fn default() -> Self {
        Self { sub_field: vec![] }
    }
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct Normal {
    field: Custom,
}

#[test]
fn normal_valid() {
    assert_eq!(
        Normal::from_json("{\"field\": {\"sub_field\": [null, 7]}}".as_bytes()).unwrap(),
        Normal {
            field: Custom {
                sub_field: vec![Null::Null, Null::Some(7)]
            },
        }
    );

    assert_eq!(
        Normal::from_json_lang("{\"field\": {\"sub_field\": [null, 7]}}".as_bytes(), "en").unwrap(),
        Normal {
            field: Custom {
                sub_field: vec![Null::Null, Null::Some(7)]
            },
        }
    );

    assert_eq!(
        Normal::from_json_stack_errs("{\"field\": {\"sub_field\": [null, 7]}}".as_bytes()).unwrap(),
        Normal {
            field: Custom {
                sub_field: vec![Null::Null, Null::Some(7)]
            },
        }
    );

    assert_eq!(
        Normal::from_json_stack_errs_lang(
            "{\"field\": {\"sub_field\": [null, 7]}}".as_bytes(),
            "en"
        )
        .unwrap(),
        Normal {
            field: Custom {
                sub_field: vec![Null::Null, Null::Some(7)]
            },
        }
    );
}

#[test]
fn normal_invalid() {
    assert_eq!(
        Normal::from_json("{\"field\": {\"sub_field\": [null, 79, null]}}".as_bytes()).unwrap_err(),
        from::Err::new_validation_err(
            From::from("sub_field"),
            vec![From::from("field")],
            "array must not contains more than 2 elements"
        ),
    );

    assert_eq!(
        Normal::from_json_lang(
            "{\"field\": {\"sub_field\": [null, 7, 78]}}".as_bytes(),
            "en"
        )
        .unwrap_err(),
        from::Err::new_validation_err(
            From::from("sub_field"),
            vec![From::from("field")],
            "array must not contains more than 2 elements"
        ),
    );

    assert_eq!(
        Normal::from_json_stack_errs("{\"field\": {\"sub_field\": [null, null, null]}}".as_bytes())
            .unwrap_err(),
        from::Errs::new_validation_err(
            From::from("sub_field"),
            vec![From::from("field")],
            "array must not contains more than 2 elements"
        ),
    );

    assert_eq!(
        Normal::from_json_stack_errs_lang(
            "{\"field\": {\"sub_field\": [7, 7, null]}}".as_bytes(),
            "en"
        )
        .unwrap_err(),
        from::Errs::new_validation_err(
            From::from("sub_field"),
            vec![From::from("field")],
            "array must not contains more than 2 elements"
        ),
    );
}

struct Validator;

impl ::from::Validator<Custom> for Validator {
    fn none(val: &Custom, path: &Path) -> Result<(), ValidationErr> {
        if val.sub_field.len() == 2 {
            return Err(ValidationErr::new(
                From::from("sub_field"),
                path.clone(),
                "test",
            ));
        };

        Ok(())
    }

    fn lang(val: &Custom, path: &Path, lang: &str) -> Result<(), ValidationErr> {
        if val.sub_field.len() == 2 {
            return Err(ValidationErr::new(
                From::from("sub_field"),
                path.clone(),
                match lang {
                    "ar" => "اختبار",
                    "en" | _ => "test",
                },
            ));
        };

        Ok(())
    }

    fn stack_errs(val: &Custom, path: &Path) -> Result<(), Vec<ValidationErr>> {
        if val.sub_field.len() == 2 {
            return Err(vec![ValidationErr::new(
                From::from("sub_field"),
                path.clone(),
                "test",
            )]);
        };

        Ok(())
    }

    fn stack_errs_lang(val: &Custom, path: &Path, lang: &str) -> Result<(), Vec<ValidationErr>> {
        if val.sub_field.len() == 2 {
            return Err(vec![ValidationErr::new(
                From::from("sub_field"),
                path.clone(),
                match lang {
                    "ar" => "اختبار",
                    "en" | _ => "test",
                },
            )]);
        };

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct CustomValidator {
    #[validators(Validator)]
    field: Custom,
}

#[test]
fn custom_validator_valid() {
    assert_eq!(
        CustomValidator::from_json(r#"{"field": {"sub_field": [24]}}"#.as_bytes()).unwrap(),
        CustomValidator {
            field: Custom {
                sub_field: vec![Null::Some(24)]
            }
        }
    );

    assert_eq!(
        CustomValidator::from_json_lang(r#"{"field":{"sub_field": [88]}}"#.as_bytes(), "en")
            .unwrap(),
        CustomValidator {
            field: Custom {
                sub_field: vec![Null::Some(88)]
            }
        }
    );

    assert_eq!(
        CustomValidator::from_json_stack_errs(r#"{"field": {"sub_field":[24]}}"#.as_bytes())
            .unwrap(),
        CustomValidator {
            field: Custom {
                sub_field: vec![Null::Some(24)]
            }
        }
    );

    assert_eq!(
        CustomValidator::from_json_stack_errs_lang(
            r#"{"field": {"sub_field":[null]}}"#.as_bytes(),
            "en"
        )
        .unwrap(),
        CustomValidator {
            field: Custom {
                sub_field: vec![Null::Null]
            }
        }
    );
}

#[test]
fn custom_validator_invalid() {
    assert_eq!(
        CustomValidator::from_json(r#"{"field": {"sub_field": [24, null]}}"#.as_bytes())
            .unwrap_err(),
        from::Err::new_validation_err(From::from("sub_field"), vec![From::from("field")], "test")
    );

    assert_eq!(
        CustomValidator::from_json_lang(r#"{"field":{"sub_field": [88, 77]}}"#.as_bytes(), "ar")
            .unwrap_err(),
        from::Err::new_validation_err(From::from("sub_field"), vec![From::from("field")], "اختبار")
    );

    assert_eq!(
        CustomValidator::from_json_stack_errs(r#"{"field": {"sub_field":[null, 95]}}"#.as_bytes())
            .unwrap_err(),
        from::Errs::new_validation_err(From::from("sub_field"), vec![From::from("field")], "test")
    );

    assert_eq!(
        CustomValidator::from_json_stack_errs_lang(
            r#"{"field": {"sub_field":[10, 12]}}"#.as_bytes(),
            "ar"
        )
        .unwrap_err(),
        from::Errs::new_validation_err(
            From::from("sub_field"),
            vec![From::from("field")],
            "اختبار"
        )
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct Required {
    field: Custom,
}

#[test]
fn required() {
    assert_eq!(
        Required::from_json(r#"{"field1": "value"}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "required field",)
    );

    assert_eq!(
        Required::from_json_lang(r#"{"field2": "value"}"#.as_bytes(), "en").unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "required field",)
    );

    assert_eq!(
        Required::from_json_stack_errs(r#"{"field4": "value"}"#.as_bytes()).unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "required field",)
    );

    assert_eq!(
        Required::from_json_stack_errs_lang(r#"{"field7": "value"}"#.as_bytes(), "en").unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "required field",)
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct RequiredOverwriteMsg {
    #[required_msgs{en = "value is required", ar="الحقل مطلوب"}]
    field: Custom,
}

#[test]
fn required_overwrite_msg() {
    assert_eq!(
        RequiredOverwriteMsg::from_json(r#"{"field7": "value"}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "value is required",)
    );

    assert_eq!(
        RequiredOverwriteMsg::from_json_lang(r#"{"field9": "value"}"#.as_bytes(), "en")
            .unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "value is required",)
    );

    assert_eq!(
        RequiredOverwriteMsg::from_json_stack_errs(r#"{}"#.as_bytes()).unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "value is required",)
    );

    assert_eq!(
        RequiredOverwriteMsg::from_json_stack_errs_lang(r#"{  }"#.as_bytes(), "ar").unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "الحقل مطلوب",)
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
#[dflt_lang = "ar"]
struct RequiredOverwriteMsgDfltLang {
    #[required_msgs{en = "value is required", ar="الحقل مطلوب"}]
    field: Custom,
}

#[test]
fn required_overwrite_msg_dflt_lang() {
    assert_eq!(
        RequiredOverwriteMsgDfltLang::from_json(r#"{"field7": "value"}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "الحقل مطلوب",)
    );

    assert_eq!(
        RequiredOverwriteMsgDfltLang::from_json_lang(r#"{"field9": "value"}"#.as_bytes(), "en")
            .unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "value is required",)
    );

    assert_eq!(
        RequiredOverwriteMsgDfltLang::from_json_stack_errs(r#"{}"#.as_bytes()).unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "الحقل مطلوب",)
    );

    assert_eq!(
        RequiredOverwriteMsgDfltLang::from_json_stack_errs_lang(r#"{  }"#.as_bytes(), "ar")
            .unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "الحقل مطلوب",)
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct NotNull {
    field: Custom,
}

#[test]
fn not_null() {
    assert_eq!(
        NotNull::from_json(r#"{"field": null}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: object, found: null",
        )
    );

    assert_eq!(
        NotNull::from_json_lang(r#"{"field": null}"#.as_bytes(), "ar").unwrap_err(),
        from::Err::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: object, found: null",
        )
    );

    assert_eq!(
        NotNull::from_json_stack_errs(r#"{"field": null}"#.as_bytes()).unwrap_err(),
        from::Errs::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: object, found: null",
        )
    );

    assert_eq!(
        NotNull::from_json_stack_errs_lang(r#"{"field": null}"#.as_bytes(), "en").unwrap_err(),
        from::Errs::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: object, found: null",
        )
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct NotNullOverwriteMsg {
    #[not_null_msgs{en="null is not allowed", ar="القيمة الخالية غير مسموحة"}]
    field: Custom,
}

#[test]
fn not_null_overwrite_msg() {
    assert_eq!(
        NotNullOverwriteMsg::from_json(r#"{"field": null}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "null is not allowed",)
    );

    assert_eq!(
        NotNullOverwriteMsg::from_json_lang(r#"{"field": null}"#.as_bytes(), "ar").unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "القيمة الخالية غير مسموحة",)
    );

    assert_eq!(
        NotNullOverwriteMsg::from_json_stack_errs(r#"{"field": null}"#.as_bytes()).unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "null is not allowed",)
    );

    assert_eq!(
        NotNullOverwriteMsg::from_json_stack_errs_lang(r#"{"field": null}"#.as_bytes(), "en")
            .unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "null is not allowed",)
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
#[dflt_lang = "ar"]
struct NotNullOverwriteMsgDfltLang {
    #[not_null_msgs{en="null is not allowed", ar="القيمة الخالية غير مسموحة"}]
    field: Custom,
}

#[test]
fn not_null_overwrite_msg_dflt_lang() {
    assert_eq!(
        NotNullOverwriteMsgDfltLang::from_json(r#"{"field": null}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "القيمة الخالية غير مسموحة",)
    );

    assert_eq!(
        NotNullOverwriteMsgDfltLang::from_json_lang(r#"{"field": null}"#.as_bytes(), "ar")
            .unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "القيمة الخالية غير مسموحة",)
    );

    assert_eq!(
        NotNullOverwriteMsgDfltLang::from_json_stack_errs(r#"{"field": null}"#.as_bytes())
            .unwrap_err(),
        from::Errs::new_validation_err(
            From::from("field"),
            Vec::new(),
            "القيمة الخالية غير مسموحة",
        )
    );

    assert_eq!(
        NotNullOverwriteMsgDfltLang::from_json_stack_errs_lang(
            r#"{"field": null}"#.as_bytes(),
            "en"
        )
        .unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "null is not allowed",)
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct NullValue {
    field: Null<Custom>,
}

#[test]
fn null() {
    assert_eq!(
        NullValue::from_json(r#"{"field": null}"#.as_bytes()).unwrap(),
        NullValue { field: Null::Null }
    );

    assert_eq!(
        NullValue::from_json(r#"{"field": {"sub_field": [null]}}"#.as_bytes()).unwrap(),
        NullValue {
            field: Null::Some(Custom {
                sub_field: vec![Null::Null]
            }),
        }
    );

    assert_eq!(
        NullValue::from_json_lang(r#"{"field": null}"#.as_bytes(), "en").unwrap(),
        NullValue { field: Null::Null }
    );

    assert_eq!(
        NullValue::from_json_lang(r#"{"field": {"sub_field": [15]}}"#.as_bytes(), "en").unwrap(),
        NullValue {
            field: Null::Some(Custom {
                sub_field: vec![Null::Some(15)]
            }),
        }
    );

    assert_eq!(
        NullValue::from_json_stack_errs(r#"{"field": null}"#.as_bytes()).unwrap(),
        NullValue { field: Null::Null }
    );

    assert_eq!(
        NullValue::from_json_stack_errs(r#"{"field": {"sub_field": []}}"#.as_bytes()).unwrap(),
        NullValue {
            field: Null::Some(Custom { sub_field: vec![] }),
        }
    );

    assert_eq!(
        NullValue::from_json_stack_errs_lang(r#"{"field": null}"#.as_bytes(), "ar").unwrap(),
        NullValue { field: Null::Null }
    );

    assert_eq!(
        NullValue::from_json_stack_errs_lang(
            r#"{"field": {"sub_field": [15, null]}}"#.as_bytes(),
            "ar"
        )
        .unwrap(),
        NullValue {
            field: Null::Some(Custom {
                sub_field: vec![Null::Some(15), Null::Null]
            }),
        }
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct OptionValue {
    field: Option<Custom>,
}

#[test]
fn option() {
    assert_eq!(
        OptionValue::from_json(r#"{"field7": null}"#.as_bytes()).unwrap(),
        OptionValue { field: None }
    );

    assert_eq!(
        OptionValue::from_json(r#"{"field": {"sub_field":[]}}"#.as_bytes()).unwrap(),
        OptionValue {
            field: Some(Custom { sub_field: vec![] }),
        }
    );

    assert_eq!(
        OptionValue::from_json_lang(r#"{"fiel3d": null}"#.as_bytes(), "en").unwrap(),
        OptionValue { field: None }
    );

    assert_eq!(
        OptionValue::from_json_lang(r#"{"field": {"sub_field":[]}}"#.as_bytes(), "en").unwrap(),
        OptionValue {
            field: Some(Custom { sub_field: vec![] }),
        }
    );

    assert_eq!(
        OptionValue::from_json_stack_errs(r#"{"fiel3d": null}"#.as_bytes()).unwrap(),
        OptionValue { field: None }
    );

    assert_eq!(
        OptionValue::from_json_stack_errs(r#"{"field": {"sub_field":[]}}"#.as_bytes()).unwrap(),
        OptionValue {
            field: Some(Custom { sub_field: vec![] }),
        }
    );

    assert_eq!(
        OptionValue::from_json_stack_errs_lang(r#"{"fi7eld": null}"#.as_bytes(), "ar").unwrap(),
        OptionValue { field: None }
    );

    assert_eq!(
        OptionValue::from_json_stack_errs_lang(r#"{"field": {"sub_field":[]}}"#.as_bytes(), "ar")
            .unwrap(),
        OptionValue {
            field: Some(Custom { sub_field: vec![] }),
        }
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct OptionNullValue {
    field: OptionNull<Custom>,
}

#[test]
fn option_null() {
    assert_eq!(
        OptionNullValue::from_json(r#"{}"#.as_bytes()).unwrap(),
        OptionNullValue {
            field: OptionNull::None
        }
    );

    assert_eq!(
        OptionNullValue::from_json(r#"{"field": null}"#.as_bytes()).unwrap(),
        OptionNullValue {
            field: OptionNull::Null
        }
    );

    assert_eq!(
        OptionNullValue::from_json(r#"{"field": {"sub_field":[]}}"#.as_bytes()).unwrap(),
        OptionNullValue {
            field: OptionNull::Some(Custom { sub_field: vec![] }),
        }
    );

    //
    //

    assert_eq!(
        OptionNullValue::from_json_lang(r#"{}"#.as_bytes(), "ar").unwrap(),
        OptionNullValue {
            field: OptionNull::None
        }
    );

    assert_eq!(
        OptionNullValue::from_json_lang(r#"{"field": null}"#.as_bytes(), "ar").unwrap(),
        OptionNullValue {
            field: OptionNull::Null
        }
    );

    assert_eq!(
        OptionNullValue::from_json_lang(r#"{"field": {"sub_field":[]}}"#.as_bytes(), "ar").unwrap(),
        OptionNullValue {
            field: OptionNull::Some(Custom { sub_field: vec![] }),
        }
    );

    //
    //

    assert_eq!(
        OptionNullValue::from_json_stack_errs(r#"{}"#.as_bytes()).unwrap(),
        OptionNullValue {
            field: OptionNull::None
        }
    );

    assert_eq!(
        OptionNullValue::from_json_stack_errs(r#"{"field": null}"#.as_bytes()).unwrap(),
        OptionNullValue {
            field: OptionNull::Null
        }
    );

    assert_eq!(
        OptionNullValue::from_json_stack_errs(r#"{"field": {"sub_field":[]}}"#.as_bytes()).unwrap(),
        OptionNullValue {
            field: OptionNull::Some(Custom { sub_field: vec![] }),
        }
    );

    //
    //

    assert_eq!(
        OptionNullValue::from_json_stack_errs_lang(r#"{}"#.as_bytes(), "en").unwrap(),
        OptionNullValue {
            field: OptionNull::None
        }
    );

    assert_eq!(
        OptionNullValue::from_json_stack_errs_lang(r#"{"field": null}"#.as_bytes(), "en").unwrap(),
        OptionNullValue {
            field: OptionNull::Null
        }
    );

    assert_eq!(
        OptionNullValue::from_json_stack_errs_lang(
            r#"{"field": {"sub_field":[]}}"#.as_bytes(),
            "en"
        )
        .unwrap(),
        OptionNullValue {
            field: OptionNull::Some(Custom { sub_field: vec![] }),
        }
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct DefaultValue {
    #[default = Default::default]
    field: Custom,
}

impl Default for DefaultValue {
    fn default() -> Self {
        Self {
            field: Default::default(),
        }
    }
}

#[test]
fn default() {
    assert_eq!(
        DefaultValue::from_json(r#"{}"#.as_bytes()).unwrap(),
        DefaultValue {
            field: Default::default(),
        }
    );

    assert_eq!(
        DefaultValue::from_json_lang(r#"{}"#.as_bytes(), "ar").unwrap(),
        DefaultValue {
            field: Default::default(),
        }
    );

    assert_eq!(
        DefaultValue::from_json_stack_errs(r#"{}"#.as_bytes()).unwrap(),
        DefaultValue {
            field: Default::default(),
        }
    );

    assert_eq!(
        DefaultValue::from_json_stack_errs_lang(r#"{}"#.as_bytes(), "en").unwrap(),
        DefaultValue {
            field: Default::default(),
        }
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct DefaultNullValue {
    #[default = null]
    field: Null<Custom>,
}

#[test]
fn default_null() {
    assert_eq!(
        DefaultNullValue::from_json(r#"{}"#.as_bytes()).unwrap(),
        DefaultNullValue { field: Null::Null }
    );

    assert_eq!(
        DefaultNullValue::from_json_lang(r#"{}"#.as_bytes(), "ar").unwrap(),
        DefaultNullValue { field: Null::Null }
    );

    assert_eq!(
        DefaultNullValue::from_json_stack_errs(r#"{}"#.as_bytes()).unwrap(),
        DefaultNullValue { field: Null::Null }
    );

    assert_eq!(
        DefaultNullValue::from_json_stack_errs_lang(r#"{}"#.as_bytes(), "en").unwrap(),
        DefaultNullValue { field: Null::Null }
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct MultiErr {
    field1: Custom,
    field2: Custom,
}

#[test]
fn multi_err() {
    assert_eq!(
        MultiErr::from_json_stack_errs(r#"{}"#.as_bytes()).unwrap_err(),
        from::Errs::ValidationErrs(vec![
            ValidationErr::new(From::from("field1"), Vec::new(), "required field"),
            ValidationErr::new(From::from("field2"), Vec::new(), "required field"),
        ])
    );

    assert_eq!(
        MultiErr::from_json_stack_errs_lang(r#"{}"#.as_bytes(), "en").unwrap_err(),
        from::Errs::ValidationErrs(vec![
            ValidationErr::new(From::from("field1"), Vec::new(), "required field"),
            ValidationErr::new(From::from("field2"), Vec::new(), "required field"),
        ])
    );
}
