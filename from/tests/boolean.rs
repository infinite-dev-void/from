use from::{from, FromJson, Null, OptionNull, Path, ValidationErr};

#[derive(Debug, PartialEq)]
#[from(json)]
struct Normal {
    field: bool,
}

#[test]
fn normal() {
    assert_eq!(
        Normal::from_json("{\"field\": true}".as_bytes()).unwrap(),
        Normal { field: true }
    );

    assert_eq!(
        Normal::from_json_lang("{\"field\": false}".as_bytes(), "en").unwrap(),
        Normal { field: false }
    );

    assert_eq!(
        Normal::from_json_stack_errs("{\"field\": true}".as_bytes()).unwrap(),
        Normal { field: true }
    );

    assert_eq!(
        Normal::from_json_stack_errs_lang("{\"field\": true}".as_bytes(), "en").unwrap(),
        Normal { field: true }
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct MustBe {
    #[must_be(value = false)]
    field: bool,
}

#[test]
fn must_be_valid() {
    assert_eq!(
        MustBe::from_json("{\"field\": false}".as_bytes()).unwrap(),
        MustBe { field: false }
    );

    assert_eq!(
        MustBe::from_json_lang("{\"field\": false}".as_bytes(), "en").unwrap(),
        MustBe { field: false }
    );

    assert_eq!(
        MustBe::from_json_stack_errs("{\"field\": false}".as_bytes()).unwrap(),
        MustBe { field: false }
    );

    assert_eq!(
        MustBe::from_json_stack_errs_lang("{\"field\": false}".as_bytes(), "en").unwrap(),
        MustBe { field: false }
    );
}

#[test]
fn must_be_invalid() {
    assert_eq!(
        MustBe::from_json(r#"{"field": true}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "value must be false",)
    );

    assert_eq!(
        MustBe::from_json_lang(r#"{"field": true}"#.as_bytes(), "en").unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "value must be false",)
    );

    assert_eq!(
        MustBe::from_json_stack_errs(r#"{"field": true}"#.as_bytes()).unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "value must be false",)
    );

    assert_eq!(
        MustBe::from_json_stack_errs_lang(r#"{"field": true}"#.as_bytes(), "en").unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "value must be false",)
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct MustBeOverwriteMsg {
    #[must_be(value = true, msgs{en="must be true", ar="يجب أن يكون صحيحا"})]
    field: bool,
}

#[test]
fn must_be_overwrite_msg_valid() {
    assert_eq!(
        MustBeOverwriteMsg::from_json("{\"field\": true}".as_bytes()).unwrap(),
        MustBeOverwriteMsg { field: true }
    );

    assert_eq!(
        MustBeOverwriteMsg::from_json_lang("{\"field\": true}".as_bytes(), "en").unwrap(),
        MustBeOverwriteMsg { field: true }
    );

    assert_eq!(
        MustBeOverwriteMsg::from_json_stack_errs("{\"field\": true}".as_bytes()).unwrap(),
        MustBeOverwriteMsg { field: true }
    );

    assert_eq!(
        MustBeOverwriteMsg::from_json_stack_errs_lang("{\"field\": true}".as_bytes(), "en")
            .unwrap(),
        MustBeOverwriteMsg { field: true }
    );
}

#[test]
fn must_be_overwrite_msg_invalid() {
    assert_eq!(
        MustBeOverwriteMsg::from_json(r#"{"field": false}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "must be true",)
    );

    assert_eq!(
        MustBeOverwriteMsg::from_json_lang(r#"{"field": false}"#.as_bytes(), "ar").unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "يجب أن يكون صحيحا",)
    );

    assert_eq!(
        MustBeOverwriteMsg::from_json_stack_errs(r#"{"field": false}"#.as_bytes()).unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "must be true",)
    );

    assert_eq!(
        MustBeOverwriteMsg::from_json_stack_errs_lang(r#"{"field": false}"#.as_bytes(), "en")
            .unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "must be true",)
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
#[dflt_lang = "ar"]
struct MustBeOverwriteMsgDfltLang {
    #[must_be(value = true, msgs{en="must be true", ar="يجب أن يكون صحيحا"})]
    field: bool,
}

#[test]
fn must_be_overwrite_msg_dflt_lang_valid() {
    assert_eq!(
        MustBeOverwriteMsgDfltLang::from_json("{\"field\": true}".as_bytes()).unwrap(),
        MustBeOverwriteMsgDfltLang { field: true }
    );

    assert_eq!(
        MustBeOverwriteMsgDfltLang::from_json_lang("{\"field\": true}".as_bytes(), "en").unwrap(),
        MustBeOverwriteMsgDfltLang { field: true }
    );

    assert_eq!(
        MustBeOverwriteMsgDfltLang::from_json_stack_errs("{\"field\": true}".as_bytes()).unwrap(),
        MustBeOverwriteMsgDfltLang { field: true }
    );

    assert_eq!(
        MustBeOverwriteMsgDfltLang::from_json_stack_errs_lang("{\"field\": true}".as_bytes(), "en")
            .unwrap(),
        MustBeOverwriteMsgDfltLang { field: true }
    );
}

#[test]
fn must_be_overwrite_msg_dflt_lang_invalid() {
    assert_eq!(
        MustBeOverwriteMsgDfltLang::from_json(r#"{"field": false}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "يجب أن يكون صحيحا",)
    );

    assert_eq!(
        MustBeOverwriteMsgDfltLang::from_json_lang(r#"{"field": false}"#.as_bytes(), "en")
            .unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "must be true",)
    );

    assert_eq!(
        MustBeOverwriteMsgDfltLang::from_json_stack_errs(r#"{"field": false}"#.as_bytes())
            .unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "يجب أن يكون صحيحا",)
    );

    assert_eq!(
        MustBeOverwriteMsgDfltLang::from_json_stack_errs_lang(
            r#"{"field": false}"#.as_bytes(),
            "en"
        )
        .unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "must be true",)
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct Required {
    field: bool,
}

#[test]
fn required() {
    assert_eq!(
        Required::from_json(r#"{"field1": true}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "required field",)
    );

    assert_eq!(
        Required::from_json_lang(r#"{"field2": true}"#.as_bytes(), "en").unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "required field",)
    );

    assert_eq!(
        Required::from_json_stack_errs(r#"{"field4": true}"#.as_bytes()).unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "required field",)
    );

    assert_eq!(
        Required::from_json_stack_errs_lang(r#"{"field7": true}"#.as_bytes(), "en").unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "required field",)
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct RequiredOverwriteMsg {
    #[required_msgs{en = "value is required", ar="الحقل مطلوب"}]
    field: bool,
}

#[test]
fn required_overwrite_msg() {
    assert_eq!(
        RequiredOverwriteMsg::from_json(r#"{"field7": true}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "value is required",)
    );

    assert_eq!(
        RequiredOverwriteMsg::from_json_lang(r#"{"field9": true}"#.as_bytes(), "en").unwrap_err(),
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
    field: bool,
}

#[test]
fn required_overwrite_msg_dflt_lang() {
    assert_eq!(
        RequiredOverwriteMsgDfltLang::from_json(r#"{"field7": true}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "الحقل مطلوب",)
    );

    assert_eq!(
        RequiredOverwriteMsgDfltLang::from_json_lang(r#"{"field9": true}"#.as_bytes(), "en")
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
struct TypeMismatch {
    field: bool,
}

#[test]
fn type_mismatch() {
    assert_eq!(
        TypeMismatch::from_json(r#"{"field": 25}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: boolean, found: number",
        )
    );

    assert_eq!(
        TypeMismatch::from_json_lang(r#"{"field": 25}"#.as_bytes(), "en").unwrap_err(),
        from::Err::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: boolean, found: number",
        )
    );

    assert_eq!(
        TypeMismatch::from_json_stack_errs(r#"{"field": 25}"#.as_bytes()).unwrap_err(),
        from::Errs::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: boolean, found: number",
        )
    );

    assert_eq!(
        TypeMismatch::from_json_stack_errs_lang(r#"{"field": 25}"#.as_bytes(), "en").unwrap_err(),
        from::Errs::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: boolean, found: number",
        )
    );

    assert_eq!(
        TypeMismatch::from_json(r#"{"field": "true"}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: boolean, found: string",
        )
    );

    assert_eq!(
        TypeMismatch::from_json_lang(r#"{"field": "hi"}"#.as_bytes(), "en").unwrap_err(),
        from::Err::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: boolean, found: string",
        )
    );

    assert_eq!(
        TypeMismatch::from_json_stack_errs(r#"{"field": ""}"#.as_bytes()).unwrap_err(),
        from::Errs::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: boolean, found: string",
        )
    );

    assert_eq!(
        TypeMismatch::from_json_stack_errs_lang(r#"{"field": "false"}"#.as_bytes(), "en")
            .unwrap_err(),
        from::Errs::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: boolean, found: string",
        )
    );

    assert_eq!(
        TypeMismatch::from_json(r#"{"field": {this will be ignored}}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: boolean, found: object",
        )
    );

    assert_eq!(
        TypeMismatch::from_json_lang(
            r#"{"field": {parser will just skip the content}}"#.as_bytes(),
            "en"
        )
        .unwrap_err(),
        from::Err::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: boolean, found: object",
        )
    );

    assert_eq!(
        TypeMismatch::from_json_stack_errs(r#"{"field": {because it does not care}}"#.as_bytes())
            .unwrap_err(),
        from::Errs::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: boolean, found: object",
        )
    );

    assert_eq!(
        TypeMismatch::from_json_stack_errs_lang(
            r#"{"field": {and this will increase the performance}}"#.as_bytes(),
            "en"
        )
        .unwrap_err(),
        from::Errs::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: boolean, found: object",
        )
    );

    assert_eq!(
        TypeMismatch::from_json(r#"{"field": [this will be ignored]}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: boolean, found: array",
        )
    );

    assert_eq!(
        TypeMismatch::from_json_lang(
            r#"{"field": [parser will just skip the content]}"#.as_bytes(),
            "en"
        )
        .unwrap_err(),
        from::Err::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: boolean, found: array",
        )
    );

    assert_eq!(
        TypeMismatch::from_json_stack_errs(r#"{"field": [because it does not care]}"#.as_bytes())
            .unwrap_err(),
        from::Errs::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: boolean, found: array",
        )
    );

    assert_eq!(
        TypeMismatch::from_json_stack_errs_lang(
            r#"{"field": [and this will increase the performance]}"#.as_bytes(),
            "en"
        )
        .unwrap_err(),
        from::Errs::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: boolean, found: array",
        )
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct TypeMismatchOverwriteMsg {
    #[type_mismatch_msgs{en="invalid type", ar="نوع غير صالح"}]
    field: bool,
}

#[test]
fn type_mismatch_overwrite_msg() {
    assert_eq!(
        TypeMismatchOverwriteMsg::from_json(r#"{"field": "false"}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "invalid type",)
    );

    assert_eq!(
        TypeMismatchOverwriteMsg::from_json_lang(r#"{"field": 25}"#.as_bytes(), "ar").unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "نوع غير صالح",)
    );

    assert_eq!(
        TypeMismatchOverwriteMsg::from_json_stack_errs(r#"{"field": {}}"#.as_bytes()).unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "invalid type",)
    );

    assert_eq!(
        TypeMismatchOverwriteMsg::from_json_stack_errs_lang(
            r#"{"field": [asd[asd[]asd]asd]}"#.as_bytes(),
            "en"
        )
        .unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "invalid type",)
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
#[dflt_lang = "ar"]
struct TypeMismatchOverwriteMsgDfltLang {
    #[type_mismatch_msgs{en="invalid type", ar="نوع غير صالح"}]
    field: bool,
}

#[test]
fn type_mismatch_overwrite_msg_dflt_lang() {
    assert_eq!(
        TypeMismatchOverwriteMsgDfltLang::from_json(r#"{"field": ""}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "نوع غير صالح",)
    );

    assert_eq!(
        TypeMismatchOverwriteMsgDfltLang::from_json_lang(r#"{"field": 25}"#.as_bytes(), "ar")
            .unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "نوع غير صالح",)
    );

    assert_eq!(
        TypeMismatchOverwriteMsgDfltLang::from_json_stack_errs(r#"{"field": {}}"#.as_bytes())
            .unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "نوع غير صالح",)
    );

    assert_eq!(
        TypeMismatchOverwriteMsgDfltLang::from_json_stack_errs_lang(
            r#"{"field": [asd[asd[]asd]asd]}"#.as_bytes(),
            "en"
        )
        .unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "invalid type",)
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct NotNull {
    field: bool,
}

#[test]
fn not_null() {
    assert_eq!(
        NotNull::from_json(r#"{"field": null}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: boolean, found: null",
        )
    );

    assert_eq!(
        NotNull::from_json_lang(r#"{"field": null}"#.as_bytes(), "ar").unwrap_err(),
        from::Err::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: boolean, found: null",
        )
    );

    assert_eq!(
        NotNull::from_json_stack_errs(r#"{"field": null}"#.as_bytes()).unwrap_err(),
        from::Errs::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: boolean, found: null",
        )
    );

    assert_eq!(
        NotNull::from_json_stack_errs_lang(r#"{"field": null}"#.as_bytes(), "en").unwrap_err(),
        from::Errs::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: boolean, found: null",
        )
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct NotNullOverwriteMsg {
    #[not_null_msgs{en="null is not allowed", ar="القيمة الخالية غير مسموحة"}]
    field: bool,
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
    field: bool,
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
    field: Null<bool>,
}

#[test]
fn null() {
    assert_eq!(
        NullValue::from_json(r#"{"field": null}"#.as_bytes()).unwrap(),
        NullValue { field: Null::Null }
    );

    assert_eq!(
        NullValue::from_json(r#"{"field": true}"#.as_bytes()).unwrap(),
        NullValue {
            field: Null::Some(true),
        }
    );

    assert_eq!(
        NullValue::from_json_lang(r#"{"field": null}"#.as_bytes(), "en").unwrap(),
        NullValue { field: Null::Null }
    );

    assert_eq!(
        NullValue::from_json_lang(r#"{"field": false}"#.as_bytes(), "en").unwrap(),
        NullValue {
            field: Null::Some(false),
        }
    );

    assert_eq!(
        NullValue::from_json_stack_errs(r#"{"field": null}"#.as_bytes()).unwrap(),
        NullValue { field: Null::Null }
    );

    assert_eq!(
        NullValue::from_json_stack_errs(r#"{"field": true}"#.as_bytes()).unwrap(),
        NullValue {
            field: Null::Some(true),
        }
    );

    assert_eq!(
        NullValue::from_json_stack_errs_lang(r#"{"field": null}"#.as_bytes(), "ar").unwrap(),
        NullValue { field: Null::Null }
    );

    assert_eq!(
        NullValue::from_json_stack_errs_lang(r#"{"field": true}"#.as_bytes(), "ar").unwrap(),
        NullValue {
            field: Null::Some(true),
        }
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct OptionValue {
    field: Option<bool>,
}

#[test]
fn option() {
    assert_eq!(
        OptionValue::from_json(r#"{"field7": null}"#.as_bytes()).unwrap(),
        OptionValue { field: None }
    );

    assert_eq!(
        OptionValue::from_json(r#"{"field": true}"#.as_bytes()).unwrap(),
        OptionValue { field: Some(true) }
    );

    assert_eq!(
        OptionValue::from_json_lang(r#"{"fiel3d": null}"#.as_bytes(), "en").unwrap(),
        OptionValue { field: None }
    );

    assert_eq!(
        OptionValue::from_json_lang(r#"{"field": true}"#.as_bytes(), "en").unwrap(),
        OptionValue { field: Some(true) }
    );

    assert_eq!(
        OptionValue::from_json_stack_errs(r#"{"fiel3d": null}"#.as_bytes()).unwrap(),
        OptionValue { field: None }
    );

    assert_eq!(
        OptionValue::from_json_stack_errs(r#"{"field": true}"#.as_bytes()).unwrap(),
        OptionValue { field: Some(true) }
    );

    assert_eq!(
        OptionValue::from_json_stack_errs_lang(r#"{"fi7eld": null}"#.as_bytes(), "ar").unwrap(),
        OptionValue { field: None }
    );

    assert_eq!(
        OptionValue::from_json_stack_errs_lang(r#"{"field": true}"#.as_bytes(), "ar").unwrap(),
        OptionValue { field: Some(true) }
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct OptionNullValue {
    field: OptionNull<bool>,
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
        OptionNullValue::from_json(r#"{"field": true}"#.as_bytes()).unwrap(),
        OptionNullValue {
            field: OptionNull::Some(true),
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
        OptionNullValue::from_json_lang(r#"{"field": true}"#.as_bytes(), "ar").unwrap(),
        OptionNullValue {
            field: OptionNull::Some(true),
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
        OptionNullValue::from_json_stack_errs(r#"{"field": true}"#.as_bytes()).unwrap(),
        OptionNullValue {
            field: OptionNull::Some(true),
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
        OptionNullValue::from_json_stack_errs_lang(r#"{"field": true}"#.as_bytes(), "en").unwrap(),
        OptionNullValue {
            field: OptionNull::Some(true),
        }
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct DefaultValue {
    #[default = false]
    field: bool,
}

#[test]
fn default() {
    assert_eq!(
        DefaultValue::from_json(r#"{}"#.as_bytes()).unwrap(),
        DefaultValue { field: false }
    );

    assert_eq!(
        DefaultValue::from_json_lang(r#"{}"#.as_bytes(), "ar").unwrap(),
        DefaultValue { field: false }
    );

    assert_eq!(
        DefaultValue::from_json_stack_errs(r#"{}"#.as_bytes()).unwrap(),
        DefaultValue { field: false }
    );

    assert_eq!(
        DefaultValue::from_json_stack_errs_lang(r#"{}"#.as_bytes(), "en").unwrap(),
        DefaultValue { field: false }
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct DefaultNullValue {
    #[default = null]
    field: Null<bool>,
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
    field1: bool,
    field2: bool,
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

struct BeTrue;

impl ::from::Validator<bool> for BeTrue {
    fn none(val: &bool, path: &Path) -> Result<(), ValidationErr> {
        if !*val {
            return Err(ValidationErr::new(
                From::from("agreed"),
                path.clone(),
                "must be true",
            ));
        };
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct CustomValidator {
    #[validators(BeTrue)]
    agreed: bool,
}

#[test]
fn custom_validator() {
    assert_eq!(
        CustomValidator::from_json(r#"{"agreed": false}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(From::from("agreed"), Vec::new(), "must be true",)
    );

    assert_eq!(
        CustomValidator::from_json_lang(r#"{"agreed": false}"#.as_bytes(), "ar").unwrap_err(),
        from::Err::new_validation_err(From::from("agreed"), Vec::new(), "must be true",)
    );

    assert_eq!(
        CustomValidator::from_json_stack_errs(r#"{"agreed": false}"#.as_bytes()).unwrap_err(),
        from::Errs::new_validation_err(From::from("agreed"), Vec::new(), "must be true",)
    );

    assert_eq!(
        CustomValidator::from_json_stack_errs_lang(r#"{"agreed": false}"#.as_bytes(), "en")
            .unwrap_err(),
        from::Errs::new_validation_err(From::from("agreed"), Vec::new(), "must be true",)
    );
}
