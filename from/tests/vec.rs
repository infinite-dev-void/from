use from::{from, FromJson, Null, OptionNull, Path, ValidationErr};

#[derive(Debug, PartialEq)]
#[from(json)]
struct Normal {
    field: Vec<String>,
}

#[test]
fn normal() {
    assert_eq!(
        Normal::from_json("{\"field\": [\"elem\"]}".as_bytes()).unwrap(),
        Normal {
            field: vec![String::from("elem")],
        }
    );

    assert_eq!(
        Normal::from_json_lang("{\"field\": [\"elem\"]}".as_bytes(), "en").unwrap(),
        Normal {
            field: vec![String::from("elem")],
        }
    );

    assert_eq!(
        Normal::from_json_stack_errs("{\"field\": [\"elem\"]}".as_bytes()).unwrap(),
        Normal {
            field: vec![String::from("elem")],
        }
    );

    assert_eq!(
        Normal::from_json_stack_errs_lang("{\"field\": [\"elem\"]}".as_bytes(), "en").unwrap(),
        Normal {
            field: vec![String::from("elem")],
        }
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct MaxLen {
    #[max_len(value = 2)]
    field: Vec<bool>,
}

#[test]
fn max_len_valid() {
    assert_eq!(
        MaxLen::from_json("{\"field\": [true, false]}".as_bytes()).unwrap(),
        MaxLen {
            field: vec![true, false],
        }
    );

    assert_eq!(
        MaxLen::from_json_lang("{\"field\": [true, false]}".as_bytes(), "en").unwrap(),
        MaxLen {
            field: vec![true, false],
        }
    );

    assert_eq!(
        MaxLen::from_json_stack_errs("{\"field\": [true, false]}".as_bytes()).unwrap(),
        MaxLen {
            field: vec![true, false],
        }
    );

    assert_eq!(
        MaxLen::from_json_stack_errs_lang("{\"field\": [true, false]}".as_bytes(), "en").unwrap(),
        MaxLen {
            field: vec![true, false],
        }
    );
}

#[test]
fn max_len_invalid() {
    assert_eq!(
        MaxLen::from_json(r#"{"field": [true, true, false]}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(
            From::from("field"),
            Vec::new(),
            "array must not contains more than 2 elements",
        )
    );

    assert_eq!(
        MaxLen::from_json_lang(r#"{"field": [true, true, false]}"#.as_bytes(), "en").unwrap_err(),
        from::Err::new_validation_err(
            From::from("field"),
            Vec::new(),
            "array must not contains more than 2 elements",
        )
    );

    assert_eq!(
        MaxLen::from_json_stack_errs(r#"{"field": [true, true, false]}"#.as_bytes()).unwrap_err(),
        from::Errs::new_validation_err(
            From::from("field"),
            Vec::new(),
            "array must not contains more than 2 elements",
        )
    );

    assert_eq!(
        MaxLen::from_json_stack_errs_lang(r#"{"field": [true, true, false]}"#.as_bytes(), "en")
            .unwrap_err(),
        from::Errs::new_validation_err(
            From::from("field"),
            Vec::new(),
            "array must not contains more than 2 elements",
        )
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct MaxLenOverwriteMsg {
    #[max_len(value = 2, msgs{en="too many elements", ar="عناصر كثيرة جداً"})]
    field: Vec<bool>,
}

#[test]
fn max_len_overwrite_msg_valid() {
    assert_eq!(
        MaxLenOverwriteMsg::from_json("{\"field\": [true, false]}".as_bytes()).unwrap(),
        MaxLenOverwriteMsg {
            field: vec![true, false],
        }
    );

    assert_eq!(
        MaxLenOverwriteMsg::from_json_lang("{\"field\": [true, false]}".as_bytes(), "en").unwrap(),
        MaxLenOverwriteMsg {
            field: vec![true, false],
        }
    );

    assert_eq!(
        MaxLenOverwriteMsg::from_json_stack_errs("{\"field\": [true, false]}".as_bytes()).unwrap(),
        MaxLenOverwriteMsg {
            field: vec![true, false],
        }
    );

    assert_eq!(
        MaxLenOverwriteMsg::from_json_stack_errs_lang(
            "{\"field\": [true, false]}".as_bytes(),
            "en"
        )
        .unwrap(),
        MaxLenOverwriteMsg {
            field: vec![true, false],
        }
    );
}

#[test]
fn max_len_overwrite_msg_invalid() {
    assert_eq!(
        MaxLenOverwriteMsg::from_json(r#"{"field": [true, true, false]}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "too many elements",)
    );

    assert_eq!(
        MaxLenOverwriteMsg::from_json_lang(r#"{"field": [true, true, false]}"#.as_bytes(), "ar")
            .unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "عناصر كثيرة جداً",)
    );

    assert_eq!(
        MaxLenOverwriteMsg::from_json_stack_errs(r#"{"field": [true, true, false]}"#.as_bytes())
            .unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "too many elements",)
    );

    assert_eq!(
        MaxLenOverwriteMsg::from_json_stack_errs_lang(
            r#"{"field": [true, true, false]}"#.as_bytes(),
            "en"
        )
        .unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "too many elements",)
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
#[dflt_lang = "ar"]
struct MaxLenOverwriteMsgDfltLang {
    #[max_len(value = 2, msgs{en="too many elements", ar="عناصر كثيرة جداً"})]
    field: Vec<bool>,
}

#[test]
fn max_len_overwrite_msg_dflt_lang_valid() {
    assert_eq!(
        MaxLenOverwriteMsgDfltLang::from_json("{\"field\": [true, false]}".as_bytes()).unwrap(),
        MaxLenOverwriteMsgDfltLang {
            field: vec![true, false],
        }
    );

    assert_eq!(
        MaxLenOverwriteMsgDfltLang::from_json_lang("{\"field\": [true, false]}".as_bytes(), "en")
            .unwrap(),
        MaxLenOverwriteMsgDfltLang {
            field: vec![true, false],
        }
    );

    assert_eq!(
        MaxLenOverwriteMsgDfltLang::from_json_stack_errs("{\"field\": [true, false]}".as_bytes())
            .unwrap(),
        MaxLenOverwriteMsgDfltLang {
            field: vec![true, false],
        }
    );

    assert_eq!(
        MaxLenOverwriteMsgDfltLang::from_json_stack_errs_lang(
            "{\"field\": [true, false]}".as_bytes(),
            "en"
        )
        .unwrap(),
        MaxLenOverwriteMsgDfltLang {
            field: vec![true, false],
        }
    );
}

#[test]
fn max_len_overwrite_msg_dflt_lang_invalid() {
    assert_eq!(
        MaxLenOverwriteMsgDfltLang::from_json(r#"{"field": [true, true, false]}"#.as_bytes())
            .unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "عناصر كثيرة جداً",)
    );

    assert_eq!(
        MaxLenOverwriteMsgDfltLang::from_json_lang(
            r#"{"field": [true, true, false]}"#.as_bytes(),
            "en"
        )
        .unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "too many elements",)
    );

    assert_eq!(
        MaxLenOverwriteMsgDfltLang::from_json_stack_errs(
            r#"{"field": [true, true, false]}"#.as_bytes()
        )
        .unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "عناصر كثيرة جداً",)
    );

    assert_eq!(
        MaxLenOverwriteMsgDfltLang::from_json_stack_errs_lang(
            r#"{"field": [true, true, false]}"#.as_bytes(),
            "en"
        )
        .unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "too many elements",)
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct MinLen {
    #[min_len(value = 2)]
    field: Vec<u8>,
}

#[test]
fn min_len_valid() {
    assert_eq!(
        MinLen::from_json("{\"field\": [1, 2]}".as_bytes()).unwrap(),
        MinLen { field: vec![1, 2] }
    );

    assert_eq!(
        MinLen::from_json_lang("{\"field\": [1, 2]}".as_bytes(), "en").unwrap(),
        MinLen { field: vec![1, 2] }
    );

    assert_eq!(
        MinLen::from_json_stack_errs("{\"field\": [1, 2]}".as_bytes()).unwrap(),
        MinLen { field: vec![1, 2] }
    );

    assert_eq!(
        MinLen::from_json_stack_errs_lang("{\"field\": [1, 2]}".as_bytes(), "en").unwrap(),
        MinLen { field: vec![1, 2] }
    );
}

#[test]
fn min_len_invalid() {
    assert_eq!(
        MinLen::from_json(r#"{"field": [10]}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(
            From::from("field"),
            Vec::new(),
            "array must not contains less than 2 elements",
        )
    );

    assert_eq!(
        MinLen::from_json_lang(r#"{"field": [  ]}"#.as_bytes(), "en").unwrap_err(),
        from::Err::new_validation_err(
            From::from("field"),
            Vec::new(),
            "array must not contains less than 2 elements",
        )
    );

    assert_eq!(
        MinLen::from_json_stack_errs(r#"{"field": []}"#.as_bytes()).unwrap_err(),
        from::Errs::new_validation_err(
            From::from("field"),
            Vec::new(),
            "array must not contains less than 2 elements",
        )
    );

    assert_eq!(
        MinLen::from_json_stack_errs_lang(r#"{"field": [15]}"#.as_bytes(), "en").unwrap_err(),
        from::Errs::new_validation_err(
            From::from("field"),
            Vec::new(),
            "array must not contains less than 2 elements",
        )
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct MinLenOverwriteMsg {
    #[min_len(value = 2, msgs{en="too short", ar="قصير جدا"})]
    field: Vec<u16>,
}

#[test]
fn min_len_overwrite_msg_valid() {
    assert_eq!(
        MinLenOverwriteMsg::from_json("{\"field\": [1, 2]}".as_bytes()).unwrap(),
        MinLenOverwriteMsg { field: vec![1, 2] }
    );

    assert_eq!(
        MinLenOverwriteMsg::from_json_lang("{\"field\": [1, 2]}".as_bytes(), "en").unwrap(),
        MinLenOverwriteMsg { field: vec![1, 2] }
    );

    assert_eq!(
        MinLenOverwriteMsg::from_json_stack_errs("{\"field\": [1, 2]}".as_bytes()).unwrap(),
        MinLenOverwriteMsg { field: vec![1, 2] }
    );

    assert_eq!(
        MinLenOverwriteMsg::from_json_stack_errs_lang("{\"field\": [1, 2]}".as_bytes(), "en")
            .unwrap(),
        MinLenOverwriteMsg { field: vec![1, 2] }
    );
}

#[test]
fn min_len_overwrite_msg_invalid() {
    assert_eq!(
        MinLenOverwriteMsg::from_json(r#"{"field": [43]}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "too short",)
    );

    assert_eq!(
        MinLenOverwriteMsg::from_json_lang(r#"{"field": [    ]}"#.as_bytes(), "ar").unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "قصير جدا",)
    );

    assert_eq!(
        MinLenOverwriteMsg::from_json_stack_errs("{\"field\": [\n\t ]}".as_bytes()).unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "too short",)
    );

    assert_eq!(
        MinLenOverwriteMsg::from_json_stack_errs_lang("{\"field\": [\n\t15\n\t]}".as_bytes(), "en")
            .unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "too short",)
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
#[dflt_lang = "ar"]
struct MinLenOverwriteMsgDfltLang {
    #[min_len(value = 2, msgs{en="too short", ar="قصير جدا"})]
    field: Vec<u32>,
}

#[test]
fn min_len_overwrite_msg_dflt_lang_valid() {
    assert_eq!(
        MinLenOverwriteMsgDfltLang::from_json("{\"field\": [1, 2]}".as_bytes()).unwrap(),
        MinLenOverwriteMsgDfltLang { field: vec![1, 2] }
    );

    assert_eq!(
        MinLenOverwriteMsgDfltLang::from_json_lang("{\"field\": [1, 2]}".as_bytes(), "en").unwrap(),
        MinLenOverwriteMsgDfltLang { field: vec![1, 2] }
    );

    assert_eq!(
        MinLenOverwriteMsgDfltLang::from_json_stack_errs("{\"field\": [1, 2]}".as_bytes()).unwrap(),
        MinLenOverwriteMsgDfltLang { field: vec![1, 2] }
    );

    assert_eq!(
        MinLenOverwriteMsgDfltLang::from_json_stack_errs_lang(
            "{\"field\": [1, 2]}".as_bytes(),
            "en"
        )
        .unwrap(),
        MinLenOverwriteMsgDfltLang { field: vec![1, 2] }
    );
}

#[test]
fn min_len_overwrite_msg_dflt_lang_invalid() {
    assert_eq!(
        MinLenOverwriteMsgDfltLang::from_json(r#"{"field": []}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "قصير جدا",)
    );

    assert_eq!(
        MinLenOverwriteMsgDfltLang::from_json_lang(r#"{"field": []}"#.as_bytes(), "en")
            .unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "too short",)
    );

    assert_eq!(
        MinLenOverwriteMsgDfltLang::from_json_stack_errs(r#"{"field": []}"#.as_bytes())
            .unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "قصير جدا",)
    );

    assert_eq!(
        MinLenOverwriteMsgDfltLang::from_json_stack_errs_lang(r#"{"field": []}"#.as_bytes(), "ar")
            .unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "قصير جدا",)
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct Len {
    #[len(value = 2)]
    field: Vec<f32>,
}

#[test]
fn len_valid() {
    assert_eq!(
        Len::from_json("{\"field\": [1.5, 2]}".as_bytes()).unwrap(),
        Len {
            field: vec![1.5, 2.0]
        }
    );

    assert_eq!(
        Len::from_json_lang("{\"field\": [1, 2.3]}".as_bytes(), "en").unwrap(),
        Len {
            field: vec![1.0, 2.3]
        }
    );

    assert_eq!(
        Len::from_json_stack_errs("{\"field\": [1, 2]}".as_bytes()).unwrap(),
        Len {
            field: vec![1.0, 2.0]
        }
    );

    assert_eq!(
        Len::from_json_stack_errs_lang("{\"field\": [1.7, 2.9]}".as_bytes(), "en").unwrap(),
        Len {
            field: vec![1.7, 2.9]
        }
    );
}

#[test]
fn len_invalid() {
    assert_eq!(
        Len::from_json(r#"{"field": [1.5, 2.3, 4.2]}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(
            From::from("field"),
            Vec::new(),
            "array must contains 2 elements",
        )
    );

    assert_eq!(
        Len::from_json_lang(r#"{"field": [-7.3, 2.7, 33.7]}"#.as_bytes(), "en").unwrap_err(),
        from::Err::new_validation_err(
            From::from("field"),
            Vec::new(),
            "array must contains 2 elements",
        )
    );

    assert_eq!(
        Len::from_json_stack_errs(r#"{"field": [8]}"#.as_bytes()).unwrap_err(),
        from::Errs::new_validation_err(
            From::from("field"),
            Vec::new(),
            "array must contains 2 elements",
        )
    );

    assert_eq!(
        Len::from_json_stack_errs_lang(r#"{"field": []}"#.as_bytes(), "en").unwrap_err(),
        from::Errs::new_validation_err(
            From::from("field"),
            Vec::new(),
            "array must contains 2 elements",
        )
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct LenOverwriteMsg {
    #[len(value = 2, msgs{en="invalid length", ar="طول غير صالح"})]
    field: Vec<f64>,
}

#[test]
fn len_overwrite_msg_valid() {
    assert_eq!(
        LenOverwriteMsg::from_json("{\"field\": [1, 2]}".as_bytes()).unwrap(),
        LenOverwriteMsg {
            field: vec![1.0, 2.0]
        }
    );

    assert_eq!(
        LenOverwriteMsg::from_json_lang("{\"field\": [1.4, 2.5]}".as_bytes(), "en").unwrap(),
        LenOverwriteMsg {
            field: vec![1.4, 2.5]
        }
    );

    assert_eq!(
        LenOverwriteMsg::from_json_stack_errs("{\"field\": [1.4, 2.5]}".as_bytes()).unwrap(),
        LenOverwriteMsg {
            field: vec![1.4, 2.5]
        }
    );

    assert_eq!(
        LenOverwriteMsg::from_json_stack_errs_lang("{\"field\": [1.4, 2.5]}".as_bytes(), "en")
            .unwrap(),
        LenOverwriteMsg {
            field: vec![1.4, 2.5]
        }
    );
}

#[test]
fn len_overwrite_msg_invalid() {
    assert_eq!(
        LenOverwriteMsg::from_json(r#"{"field": []}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "invalid length",)
    );

    assert_eq!(
        LenOverwriteMsg::from_json_lang(r#"{"field": [5]}"#.as_bytes(), "ar").unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "طول غير صالح",)
    );

    assert_eq!(
        LenOverwriteMsg::from_json_stack_errs(r#"{"field": [7.0]}"#.as_bytes()).unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "invalid length",)
    );

    assert_eq!(
        LenOverwriteMsg::from_json_stack_errs_lang(
            r#"{"field": [-8, -15, -5.5]}"#.as_bytes(),
            "ar"
        )
        .unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "طول غير صالح",)
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
#[dflt_lang = "ar"]
struct LenOverwriteMsgDfltLang {
    #[len(value = 2, msgs{en="invalid length", ar="طول غير صالح"})]
    field: Vec<f64>,
}

#[test]
fn len_overwrite_msg_dflt_lang_valid() {
    assert_eq!(
        LenOverwriteMsgDfltLang::from_json("{\"field\": [1, 2]}".as_bytes()).unwrap(),
        LenOverwriteMsgDfltLang {
            field: vec![1f64, 2f64]
        }
    );

    assert_eq!(
        LenOverwriteMsgDfltLang::from_json_lang("{\"field\": [1, 2]}".as_bytes(), "en").unwrap(),
        LenOverwriteMsgDfltLang {
            field: vec![1f64, 2f64]
        }
    );

    assert_eq!(
        LenOverwriteMsgDfltLang::from_json_stack_errs("{\"field\": [1, 2]}".as_bytes()).unwrap(),
        LenOverwriteMsgDfltLang {
            field: vec![1f64, 2f64]
        }
    );

    assert_eq!(
        LenOverwriteMsgDfltLang::from_json_stack_errs_lang("{\"field\": [1, 2]}".as_bytes(), "en")
            .unwrap(),
        LenOverwriteMsgDfltLang {
            field: vec![1f64, 2f64]
        }
    );
}

#[test]
fn len_overwrite_msg_dflt_lang_invalid() {
    assert_eq!(
        LenOverwriteMsgDfltLang::from_json(r#"{"field": []}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "طول غير صالح",)
    );

    assert_eq!(
        LenOverwriteMsgDfltLang::from_json_lang(r#"{"field": [-8]}"#.as_bytes(), "en").unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "invalid length",)
    );

    assert_eq!(
        LenOverwriteMsgDfltLang::from_json_stack_errs(r#"{"field": [-9e3]}"#.as_bytes())
            .unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "طول غير صالح",)
    );

    assert_eq!(
        LenOverwriteMsgDfltLang::from_json_stack_errs_lang(r#"{"field": [-7e3]}"#.as_bytes(), "en")
            .unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "invalid length",)
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct Required {
    field: Vec<Null<bool>>,
}

#[test]
fn required() {
    assert_eq!(
        Required::from_json(r#"{"field1": [true, true, null]}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "required field",)
    );

    assert_eq!(
        Required::from_json_lang(r#"{"field2": [true, true, false]}"#.as_bytes(), "en")
            .unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "required field",)
    );

    assert_eq!(
        Required::from_json_stack_errs(r#"{"field4": [true, true, false]}"#.as_bytes())
            .unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "required field",)
    );

    assert_eq!(
        Required::from_json_stack_errs_lang(r#"{"field7": [true, true, false]}"#.as_bytes(), "en")
            .unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "required field",)
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct RequiredOverwriteMsg {
    #[required_msgs{en = "value is required", ar="الحقل مطلوب"}]
    field: Vec<Vec<String>>,
}

#[test]
fn required_overwrite_msg() {
    assert_eq!(
        RequiredOverwriteMsg::from_json(r#"{"field7": [true, true, false]}"#.as_bytes())
            .unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "value is required",)
    );

    assert_eq!(
        RequiredOverwriteMsg::from_json_lang(r#"{"field9": [true, true, false]}"#.as_bytes(), "en")
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
    field: Vec<Null<u8>>,
}

#[test]
fn required_overwrite_msg_dflt_lang() {
    assert_eq!(
        RequiredOverwriteMsgDfltLang::from_json(r#"{"field7": [true, true, false]}"#.as_bytes())
            .unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "الحقل مطلوب",)
    );

    assert_eq!(
        RequiredOverwriteMsgDfltLang::from_json_lang(
            r#"{"field9": [true, true, false]}"#.as_bytes(),
            "en"
        )
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
    field: Vec<Vec<Null<i16>>>,
}

#[test]
fn type_mismatch() {
    assert_eq!(
        TypeMismatch::from_json(r#"{"field": 25}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: array, found: number",
        )
    );

    assert_eq!(
        TypeMismatch::from_json_lang(r#"{"field": 25}"#.as_bytes(), "en").unwrap_err(),
        from::Err::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: array, found: number",
        )
    );

    assert_eq!(
        TypeMismatch::from_json_stack_errs(r#"{"field": 25}"#.as_bytes()).unwrap_err(),
        from::Errs::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: array, found: number",
        )
    );

    assert_eq!(
        TypeMismatch::from_json_stack_errs_lang(r#"{"field": 25}"#.as_bytes(), "en").unwrap_err(),
        from::Errs::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: array, found: number",
        )
    );

    assert_eq!(
        TypeMismatch::from_json(r#"{"field": true}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: array, found: boolean",
        )
    );

    assert_eq!(
        TypeMismatch::from_json_lang(r#"{"field": false}"#.as_bytes(), "en").unwrap_err(),
        from::Err::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: array, found: boolean",
        )
    );

    assert_eq!(
        TypeMismatch::from_json_stack_errs(r#"{"field": true}"#.as_bytes()).unwrap_err(),
        from::Errs::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: array, found: boolean",
        )
    );

    assert_eq!(
        TypeMismatch::from_json_stack_errs_lang(r#"{"field": true}"#.as_bytes(), "en").unwrap_err(),
        from::Errs::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: array, found: boolean",
        )
    );

    assert_eq!(
        TypeMismatch::from_json(r#"{"field": {this will be ignored}}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: array, found: object",
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
            "expected: array, found: object",
        )
    );

    assert_eq!(
        TypeMismatch::from_json_stack_errs(r#"{"field": {because it does not care}}"#.as_bytes())
            .unwrap_err(),
        from::Errs::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: array, found: object",
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
            "expected: array, found: object",
        )
    );

    assert_eq!(
        TypeMismatch::from_json(r#"{"field": "hello"}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: array, found: string",
        )
    );

    assert_eq!(
        TypeMismatch::from_json_lang(r#"{"field": ""}"#.as_bytes(), "en").unwrap_err(),
        from::Err::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: array, found: string",
        )
    );

    assert_eq!(
        TypeMismatch::from_json_stack_errs(r#"{"field": "noop"}"#.as_bytes()).unwrap_err(),
        from::Errs::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: array, found: string",
        )
    );

    assert_eq!(
        TypeMismatch::from_json_stack_errs_lang(r#"{"field": "hello\t"}"#.as_bytes(), "en")
            .unwrap_err(),
        from::Errs::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: array, found: string",
        )
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct TypeMismatchOverwriteMsg {
    #[type_mismatch_msgs{en="invalid type", ar="نوع غير صالح"}]
    field: Vec<Null<Vec<bool>>>,
}

#[test]
fn type_mismatch_overwrite_msg() {
    assert_eq!(
        TypeMismatchOverwriteMsg::from_json(r#"{"field": false}"#.as_bytes()).unwrap_err(),
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
        TypeMismatchOverwriteMsg::from_json_stack_errs_lang(r#"{"field": ""}"#.as_bytes(), "en")
            .unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "invalid type",)
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
#[dflt_lang = "ar"]
struct TypeMismatchOverwriteMsgDfltLang {
    #[type_mismatch_msgs{en="invalid type", ar="نوع غير صالح"}]
    field: Vec<String>,
}

#[test]
fn type_mismatch_overwrite_msg_dflt_lang() {
    assert_eq!(
        TypeMismatchOverwriteMsgDfltLang::from_json(r#"{"field": false}"#.as_bytes()).unwrap_err(),
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
            r#"{"field": ""}"#.as_bytes(),
            "en"
        )
        .unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "invalid type",)
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct NotNull {
    field: Vec<u64>,
}

#[test]
fn not_null() {
    assert_eq!(
        NotNull::from_json(r#"{"field": null}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: array, found: null",
        )
    );

    assert_eq!(
        NotNull::from_json_lang(r#"{"field": null}"#.as_bytes(), "ar").unwrap_err(),
        from::Err::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: array, found: null",
        )
    );

    assert_eq!(
        NotNull::from_json_stack_errs(r#"{"field": null}"#.as_bytes()).unwrap_err(),
        from::Errs::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: array, found: null",
        )
    );

    assert_eq!(
        NotNull::from_json_stack_errs_lang(r#"{"field": null}"#.as_bytes(), "en").unwrap_err(),
        from::Errs::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: array, found: null",
        )
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct NotNullOverwriteMsg {
    #[not_null_msgs{en="null is not allowed", ar="القيمة الخالية غير مسموحة"}]
    field: Vec<usize>,
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
    field: Vec<Null<i128>>,
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
    field: Null<Vec<Null<Vec<Null<i64>>>>>,
}

#[test]
fn null() {
    assert_eq!(
        NullValue::from_json(r#"{"field": null}"#.as_bytes()).unwrap(),
        NullValue { field: Null::Null }
    );

    assert_eq!(
        NullValue::from_json(r#"{"field": [null, [null, 65]]}"#.as_bytes()).unwrap(),
        NullValue {
            field: Null::Some(vec![
                Null::Null,
                Null::Some(vec![Null::Null, Null::Some(65)])
            ]),
        }
    );

    assert_eq!(
        NullValue::from_json_lang(r#"{"field": null}"#.as_bytes(), "en").unwrap(),
        NullValue { field: Null::Null }
    );

    assert_eq!(
        NullValue::from_json_lang(r#"{"field": [[null, 79], null]}"#.as_bytes(), "en").unwrap(),
        NullValue {
            field: Null::Some(vec![
                Null::Some(vec![Null::Null, Null::Some(79)]),
                Null::Null
            ]),
        }
    );

    assert_eq!(
        NullValue::from_json_stack_errs(r#"{"field": null}"#.as_bytes()).unwrap(),
        NullValue { field: Null::Null }
    );

    assert_eq!(
        NullValue::from_json_stack_errs(r#"{"field": [null]}"#.as_bytes()).unwrap(),
        NullValue {
            field: Null::Some(vec![Null::Null]),
        }
    );

    assert_eq!(
        NullValue::from_json_stack_errs_lang(r#"{"field": null}"#.as_bytes(), "ar").unwrap(),
        NullValue { field: Null::Null }
    );

    assert_eq!(
        NullValue::from_json_stack_errs_lang(r#"{"field": []}"#.as_bytes(), "ar").unwrap(),
        NullValue {
            field: Null::Some(vec![]),
        }
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct OptionValue {
    field: Option<Vec<Null<String>>>,
}

#[test]
fn option() {
    assert_eq!(
        OptionValue::from_json(r#"{"field7": null}"#.as_bytes()).unwrap(),
        OptionValue { field: None }
    );

    assert_eq!(
        OptionValue::from_json(r#"{"field": [null, "hi"]}"#.as_bytes()).unwrap(),
        OptionValue {
            field: Some(vec![Null::Null, Null::Some(String::from("hi"))]),
        }
    );

    assert_eq!(
        OptionValue::from_json_lang(r#"{"fiel3d": null}"#.as_bytes(), "en").unwrap(),
        OptionValue { field: None }
    );

    assert_eq!(
        OptionValue::from_json_lang(r#"{"field": [""]}"#.as_bytes(), "en").unwrap(),
        OptionValue {
            field: Some(vec![Null::Some(String::from(""))]),
        }
    );

    assert_eq!(
        OptionValue::from_json_stack_errs(r#"{"fiel3d": null}"#.as_bytes()).unwrap(),
        OptionValue { field: None }
    );

    assert_eq!(
        OptionValue::from_json_stack_errs(r#"{"field": [null]}"#.as_bytes()).unwrap(),
        OptionValue {
            field: Some(vec![Null::Null]),
        }
    );

    assert_eq!(
        OptionValue::from_json_stack_errs_lang(r#"{"fi7eld": null}"#.as_bytes(), "ar").unwrap(),
        OptionValue { field: None }
    );

    assert_eq!(
        OptionValue::from_json_stack_errs_lang(r#"{"field": []}"#.as_bytes(), "ar").unwrap(),
        OptionValue {
            field: Some(vec![]),
        }
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct OptionNullValue {
    field: OptionNull<Vec<Vec<Null<isize>>>>,
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
        OptionNullValue::from_json(r#"{"field": [[15], [-74, null]]}"#.as_bytes()).unwrap(),
        OptionNullValue {
            field: OptionNull::Some(vec![
                vec![Null::Some(15)],
                vec![Null::Some(-74), Null::Null]
            ]),
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
        OptionNullValue::from_json_lang(r#"{"field": [[] , [ ] ,  [null]]}"#.as_bytes(), "ar")
            .unwrap(),
        OptionNullValue {
            field: OptionNull::Some(vec![vec![], vec![], vec![Null::Null]]),
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
        OptionNullValue::from_json_stack_errs(r#"{"field": []}"#.as_bytes()).unwrap(),
        OptionNullValue {
            field: OptionNull::Some(vec![]),
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
            r#"{"field": [  [ ] , [ -10 ] , [null]]}"#.as_bytes(),
            "en"
        )
        .unwrap(),
        OptionNullValue {
            field: OptionNull::Some(vec![vec![], vec![Null::Some(-10)], vec![Null::Null]]),
        }
    );
}
/*
#[derive(Debug, PartialEq)]
#[from(json)]
struct DefaultValue {
    #[default = "hello"]
    field: String,
}

#[test]
fn default() {
    assert_eq!(
        DefaultValue::from_json(r#"{}"#.as_bytes()).unwrap(),
        DefaultValue {
            field: String::from("hello")
        }
    );

    assert_eq!(
        DefaultValue::from_json_lang(r#"{}"#.as_bytes(), "ar").unwrap(),
        DefaultValue {
            field: String::from("hello")
        }
    );

    assert_eq!(
        DefaultValue::from_json_stack_errs(r#"{}"#.as_bytes()).unwrap(),
        DefaultValue {
            field: String::from("hello")
        }
    );

    assert_eq!(
        DefaultValue::from_json_stack_errs_lang(r#"{}"#.as_bytes(), "en").unwrap(),
        DefaultValue {
            field: String::from("hello")
        }
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct DefaultNullValue {
    #[default = null]
    field: Null<String>,
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
 */
#[derive(Debug, PartialEq)]
#[from(json)]
struct MultiErr {
    field1: Vec<u16>,
    field2: Vec<u16>,
}

#[test]
fn multi_err() {
    assert_eq!(
        MultiErr::from_json_stack_errs(r#"{}"#.as_bytes()).unwrap_err(),
        from::Errs::ValidationErrs(vec![
            from::ValidationErr::new(From::from("field1"), Vec::new(), "required field"),
            from::ValidationErr::new(From::from("field2"), Vec::new(), "required field"),
        ])
    );

    assert_eq!(
        MultiErr::from_json_stack_errs_lang(r#"{}"#.as_bytes(), "en").unwrap_err(),
        from::Errs::ValidationErrs(vec![
            from::ValidationErr::new(From::from("field1"), Vec::new(), "required field"),
            from::ValidationErr::new(From::from("field2"), Vec::new(), "required field"),
        ])
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]

struct StringElem {
    #[elem{
        max_len(value=5)
    }]
    field: Vec<String>,
}

#[test]

fn string_elem() {
    assert_eq!(
        StringElem::from_json(r#"{"field": ["nice", "hello world!"]}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(
            From::from(1),
            vec![From::from("field")],
            "the string value must be no longer than 5 bytes"
        )
    );

    assert_eq!(
        StringElem::from_json_lang(r#"{"field": ["nice", "hello world!"]}"#.as_bytes(), "en")
            .unwrap_err(),
        from::Err::new_validation_err(
            From::from(1),
            vec![From::from("field")],
            "the string value must be no longer than 5 bytes"
        )
    );

    assert_eq!(
        StringElem::from_json_stack_errs(r#"{"field": ["nice", "hello world!"]}"#.as_bytes())
            .unwrap_err(),
        from::Errs::new_validation_err(
            From::from(1),
            vec![From::from("field")],
            "the string value must be no longer than 5 bytes"
        )
    );

    assert_eq!(
        StringElem::from_json_stack_errs_lang(
            r#"{"field": ["nice", "hello world!"]}"#.as_bytes(),
            "en"
        )
        .unwrap_err(),
        from::Errs::new_validation_err(
            From::from(1),
            vec![From::from("field")],
            "the string value must be no longer than 5 bytes"
        )
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]

struct FloatElem {
    #[elem{
        max(value=5.0)
    }]
    field: Vec<f32>,
}

#[test]

fn float_elem() {
    assert_eq!(
        FloatElem::from_json(r#"{"field": [4, 7.4]}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(
            From::from(1),
            vec![From::from("field")],
            "number must be less than or equal to 5"
        )
    );

    assert_eq!(
        FloatElem::from_json_lang(r#"{"field": [4, 7.4]}"#.as_bytes(), "en").unwrap_err(),
        from::Err::new_validation_err(
            From::from(1),
            vec![From::from("field")],
            "number must be less than or equal to 5"
        )
    );

    assert_eq!(
        FloatElem::from_json_stack_errs(r#"{"field": [4, 7.5]}"#.as_bytes()).unwrap_err(),
        from::Errs::new_validation_err(
            From::from(1),
            vec![From::from("field")],
            "number must be less than or equal to 5"
        )
    );

    assert_eq!(
        FloatElem::from_json_stack_errs_lang(r#"{"field": [4, 7.5]}"#.as_bytes(), "en")
            .unwrap_err(),
        from::Errs::new_validation_err(
            From::from(1),
            vec![From::from("field")],
            "number must be less than or equal to 5"
        )
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]

struct IntElem {
    #[elem{
        max(value=7)
    }]
    field: Vec<usize>,
}

#[test]

fn int_elem() {
    assert_eq!(
        IntElem::from_json(r#"{"field": [7, 5, 15]}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(
            From::from(2),
            vec![From::from("field")],
            "number must be less than or equal to 7"
        )
    );

    assert_eq!(
        IntElem::from_json_lang(r#"{"field": [20]}"#.as_bytes(), "en").unwrap_err(),
        from::Err::new_validation_err(
            From::from(0),
            vec![From::from("field")],
            "number must be less than or equal to 7"
        )
    );

    assert_eq!(
        IntElem::from_json_stack_errs(r#"{"field": [4, 3, 7, 15]}"#.as_bytes()).unwrap_err(),
        from::Errs::new_validation_err(
            From::from(3),
            vec![From::from("field")],
            "number must be less than or equal to 7"
        )
    );

    assert_eq!(
        IntElem::from_json_stack_errs_lang(r#"{"field": [1,2 ,3, 4 ,5 ,7, 10]}"#.as_bytes(), "en")
            .unwrap_err(),
        from::Errs::new_validation_err(
            From::from(6),
            vec![From::from("field")],
            "number must be less than or equal to 7"
        )
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]

struct BoolElem {
    #[elem{
        must_be(value = true)
    }]
    field: Vec<bool>,
}

#[test]

fn bool_elem() {
    assert_eq!(
        BoolElem::from_json(r#"{"field": [true, true, false]}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(
            From::from(2),
            vec![From::from("field")],
            "value must be true"
        )
    );

    assert_eq!(
        BoolElem::from_json_lang(r#"{"field": [false]}"#.as_bytes(), "en").unwrap_err(),
        from::Err::new_validation_err(
            From::from(0),
            vec![From::from("field")],
            "value must be true"
        )
    );

    assert_eq!(
        BoolElem::from_json_stack_errs(r#"{"field": [true, true, true, false]}"#.as_bytes())
            .unwrap_err(),
        from::Errs::new_validation_err(
            From::from(3),
            vec![From::from("field")],
            "value must be true"
        )
    );

    assert_eq!(
        BoolElem::from_json_stack_errs_lang(
            r#"{"field": [true, true, true, true, true, true, false]}"#.as_bytes(),
            "en"
        )
        .unwrap_err(),
        from::Errs::new_validation_err(
            From::from(6),
            vec![From::from("field")],
            "value must be true"
        )
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]

struct VecElem {
    #[elem{
        min_len(value=1),

        elem{
            must_be(value=true)
        }
    }]
    field: Vec<Vec<bool>>,
}

#[test]

fn vec_elem() {
    assert_eq!(
        VecElem::from_json(r#"{"field": [[]]}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(
            From::from(0),
            vec![From::from("field")],
            "array must not contains less than 1 element"
        )
    );

    assert_eq!(
        VecElem::from_json_lang(r#"{"field": [[true], []]}"#.as_bytes(), "en").unwrap_err(),
        from::Err::new_validation_err(
            From::from(1),
            vec![From::from("field")],
            "array must not contains less than 1 element"
        )
    );

    assert_eq!(
        VecElem::from_json_stack_errs(r#"{"field": [[true], [true], [true], []]}"#.as_bytes())
            .unwrap_err(),
        from::Errs::new_validation_err(
            From::from(3),
            vec![From::from("field")],
            "array must not contains less than 1 element"
        )
    );

    assert_eq!(
        VecElem::from_json_stack_errs_lang(
            r#"{"field": [[true], [true], [true], [true], [true], [true], []]}"#.as_bytes(),
            "en"
        )
        .unwrap_err(),
        from::Errs::new_validation_err(
            From::from(6),
            vec![From::from("field")],
            "array must not contains less than 1 element"
        )
    );

    assert_eq!(
        VecElem::from_json(r#"{"field": [[false]]}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(
            From::from(0),
            vec![From::from("field"), From::from(0)],
            "value must be true"
        )
    );

    assert_eq!(
        VecElem::from_json(r#"{"field": [[true], [true, false]]}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(
            From::from(1),
            vec![From::from("field"), From::from(1)],
            "value must be true"
        )
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct NullVecElem {
    #[elem{
        max_len(value=1)
    }]
    field: Vec<Null<Vec<u8>>>,
}

#[test]
fn null_vec_elem() {
    assert_eq!(
        NullVecElem::from_json(r#"{"field": [null, [1, 2]]}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(
            From::from(1),
            vec![From::from("field")],
            "array must not contains more than 1 element"
        )
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct Custom {
    sub_field: u8,
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct CustomElem {
    field: Vec<Custom>,
}

#[test]
fn custom_elem() {
    assert_eq!(
        CustomElem::from_json(r#"{"field": [{"sub_field": 15}]}"#.as_bytes()).unwrap(),
        CustomElem {
            field: vec![Custom { sub_field: 15 }]
        }
    );

    assert_eq!(
        CustomElem::from_json_lang(
            r#"{"field": [{"sub_field": 75}, {"sub_field": 45}]}"#.as_bytes(),
            "en"
        )
        .unwrap(),
        CustomElem {
            field: vec![Custom { sub_field: 75 }, Custom { sub_field: 45 }]
        }
    );

    assert_eq!(
        CustomElem::from_json_stack_errs(
            r#"{
                "field": [
                    {"sub_field": 33},
                    {"sub_field": 113},
                    {"sub_field": 78}
                ]
            }"#
            .as_bytes()
        )
        .unwrap(),
        CustomElem {
            field: vec![
                Custom { sub_field: 33 },
                Custom { sub_field: 113 },
                Custom { sub_field: 78 },
            ]
        }
    );

    assert_eq!(
        CustomElem::from_json_stack_errs_lang(r#"{"field": []}"#.as_bytes(), "en").unwrap(),
        CustomElem { field: vec![] }
    );
}

struct PhoneNumbers;

impl ::from::Validator<Vec<String>> for PhoneNumbers {
    fn none(val: &Vec<String>, path: &Path) -> Result<(), ValidationErr> {
        if val.len() == 0 {
            return Err(ValidationErr::new(
                From::from("phone_numbers"),
                path.clone(),
                "at least one phone number",
            ));
        };
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct CustomValidator {
    #[validators(PhoneNumbers)]
    phone_numbers: Vec<String>,
}

#[test]
fn custom_validator() {
    assert_eq!(
        CustomValidator::from_json(r#"{"phone_numbers": []}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(
            From::from("phone_numbers"),
            Vec::new(),
            "at least one phone number",
        )
    );

    assert_eq!(
        CustomValidator::from_json_lang(r#"{"phone_numbers": []}"#.as_bytes(), "ar").unwrap_err(),
        from::Err::new_validation_err(
            From::from("phone_numbers"),
            Vec::new(),
            "at least one phone number",
        )
    );

    assert_eq!(
        CustomValidator::from_json_stack_errs(r#"{"phone_numbers": []}"#.as_bytes()).unwrap_err(),
        from::Errs::new_validation_err(
            From::from("phone_numbers"),
            Vec::new(),
            "at least one phone number",
        )
    );

    assert_eq!(
        CustomValidator::from_json_stack_errs_lang(r#"{"phone_numbers": []}"#.as_bytes(), "en")
            .unwrap_err(),
        from::Errs::new_validation_err(
            From::from("phone_numbers"),
            Vec::new(),
            "at least one phone number",
        )
    );
}
