#[inline]
fn json_too_large(mut num: String) -> String {
    num.push('7');
    format!(r#"{{"field": {}}}"#, num)
}

#[inline]
fn json_too_small(mut num: String) -> String {
    if !num.starts_with('-') {
        num = String::from("-1");
    } else {
        num.push('8');
    }

    format!(r#"{{"field": {}}}"#, num)
}

macro_rules! ints {
    (
        typ: $typ:ident,
        typ_str: $typ_str: expr,
    ) => {
        pub mod $typ {
            use from::{from, FromJson, Null, OptionNull, Path, ValidationErr};

            use super::{json_too_large, json_too_small};

            #[derive(Debug, PartialEq)]
            #[from(json)]
            struct Normal {
                field: $typ,
            }

            #[test]
            fn normal() {
                assert_eq!(
                    Normal::from_json("{\"field\": 5}".as_bytes()).unwrap(),
                    Normal { field: 5 }
                );

                assert_eq!(
                    Normal::from_json_lang("{\"field\": 5}".as_bytes(), "en").unwrap(),
                    Normal { field: 5 }
                );

                assert_eq!(
                    Normal::from_json_stack_errs("{\"field\": 5}".as_bytes()).unwrap(),
                    Normal { field: 5 }
                );

                assert_eq!(
                    Normal::from_json_stack_errs_lang("{\"field\": 5}".as_bytes(), "en").unwrap(),
                    Normal { field: 5 }
                );
            }

            #[derive(Debug, PartialEq)]
            #[from(json)]
            struct Max {
                #[max(value = 4)]
                field: $typ,
            }

            #[test]
            fn max_valid() {
                assert_eq!(
                    Max::from_json("{\"field\": 3}".as_bytes()).unwrap(),
                    Max { field: 3 }
                );

                assert_eq!(
                    Max::from_json_lang("{\"field\": 3}".as_bytes(), "en").unwrap(),
                    Max { field: 3 }
                );

                assert_eq!(
                    Max::from_json_stack_errs("{\"field\": 3}".as_bytes()).unwrap(),
                    Max { field: 3 }
                );

                assert_eq!(
                    Max::from_json_stack_errs_lang("{\"field\": 3}".as_bytes(), "en").unwrap(),
                    Max { field: 3 }
                );
            }

            #[test]
            fn max_invalid() {
                assert_eq!(
                    Max::from_json(r#"{"field": 5}"#.as_bytes()).unwrap_err(),
                    from::Err::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        "number must be less than or equal to 4",
                    )
                );

                assert_eq!(
                    Max::from_json_lang(r#"{"field": 5}"#.as_bytes(), "en").unwrap_err(),
                    from::Err::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        "number must be less than or equal to 4",
                    )
                );

                assert_eq!(
                    Max::from_json_stack_errs(r#"{"field": 5}"#.as_bytes()).unwrap_err(),
                    from::Errs::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        "number must be less than or equal to 4",
                    )
                );

                assert_eq!(
                    Max::from_json_stack_errs_lang(r#"{"field": 5}"#.as_bytes(), "en").unwrap_err(),
                    from::Errs::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        "number must be less than or equal to 4",
                    )
                );
            }

            #[derive(Debug, PartialEq)]
            #[from(json)]
            struct MaxOverwriteMsg {
                #[max(value = 4, msgs{en="too big", ar="كبير جدا"})]
                field: $typ,
            }

            #[test]
            fn max_overwrite_msg_valid() {
                assert_eq!(
                    MaxOverwriteMsg::from_json("{\"field\": 3}".as_bytes()).unwrap(),
                    MaxOverwriteMsg { field: 3 }
                );

                assert_eq!(
                    MaxOverwriteMsg::from_json_lang("{\"field\": 3}".as_bytes(), "en").unwrap(),
                    MaxOverwriteMsg { field: 3 }
                );

                assert_eq!(
                    MaxOverwriteMsg::from_json_stack_errs("{\"field\": 3}".as_bytes()).unwrap(),
                    MaxOverwriteMsg { field: 3 }
                );

                assert_eq!(
                    MaxOverwriteMsg::from_json_stack_errs_lang("{\"field\": 3}".as_bytes(), "en")
                        .unwrap(),
                    MaxOverwriteMsg { field: 3 }
                );
            }

            #[test]
            fn max_overwrite_msg_invalid() {
                assert_eq!(
                    MaxOverwriteMsg::from_json(r#"{"field": 5}"#.as_bytes()).unwrap_err(),
                    from::Err::new_validation_err(From::from("field"), Vec::new(), "too big",)
                );

                assert_eq!(
                    MaxOverwriteMsg::from_json_lang(r#"{"field": 5}"#.as_bytes(), "ar")
                        .unwrap_err(),
                    from::Err::new_validation_err(From::from("field"), Vec::new(), "كبير جدا",)
                );

                assert_eq!(
                    MaxOverwriteMsg::from_json_stack_errs(r#"{"field": 5}"#.as_bytes())
                        .unwrap_err(),
                    from::Errs::new_validation_err(From::from("field"), Vec::new(), "too big",)
                );

                assert_eq!(
                    MaxOverwriteMsg::from_json_stack_errs_lang(r#"{"field": 5}"#.as_bytes(), "en")
                        .unwrap_err(),
                    from::Errs::new_validation_err(From::from("field"), Vec::new(), "too big",)
                );
            }

            #[derive(Debug, PartialEq)]
            #[from(json)]
            #[dflt_lang = "ar"]
            struct MaxOverwriteMsgDfltLang {
                #[max(value = 4, msgs{en="too big", ar="كبير جدا"})]
                field: $typ,
            }

            #[test]
            fn max_overwrite_msg_dflt_lang_valid() {
                assert_eq!(
                    MaxOverwriteMsgDfltLang::from_json("{\"field\": 3}".as_bytes()).unwrap(),
                    MaxOverwriteMsgDfltLang { field: 3 }
                );

                assert_eq!(
                    MaxOverwriteMsgDfltLang::from_json_lang("{\"field\": 3}".as_bytes(), "en")
                        .unwrap(),
                    MaxOverwriteMsgDfltLang { field: 3 }
                );

                assert_eq!(
                    MaxOverwriteMsgDfltLang::from_json_stack_errs("{\"field\": 3}".as_bytes())
                        .unwrap(),
                    MaxOverwriteMsgDfltLang { field: 3 }
                );

                assert_eq!(
                    MaxOverwriteMsgDfltLang::from_json_stack_errs_lang(
                        "{\"field\": 3}".as_bytes(),
                        "en"
                    )
                    .unwrap(),
                    MaxOverwriteMsgDfltLang { field: 3 }
                );
            }

            #[test]
            fn max_overwrite_msg_dflt_lang_invalid() {
                assert_eq!(
                    MaxOverwriteMsgDfltLang::from_json(r#"{"field": 5}"#.as_bytes()).unwrap_err(),
                    from::Err::new_validation_err(From::from("field"), Vec::new(), "كبير جدا",)
                );

                assert_eq!(
                    MaxOverwriteMsgDfltLang::from_json_lang(r#"{"field": 5}"#.as_bytes(), "en")
                        .unwrap_err(),
                    from::Err::new_validation_err(From::from("field"), Vec::new(), "too big",)
                );

                assert_eq!(
                    MaxOverwriteMsgDfltLang::from_json_stack_errs(r#"{"field": 5}"#.as_bytes())
                        .unwrap_err(),
                    from::Errs::new_validation_err(From::from("field"), Vec::new(), "كبير جدا",)
                );

                assert_eq!(
                    MaxOverwriteMsgDfltLang::from_json_stack_errs_lang(
                        r#"{"field": 5}"#.as_bytes(),
                        "en"
                    )
                    .unwrap_err(),
                    from::Errs::new_validation_err(From::from("field"), Vec::new(), "too big",)
                );
            }

            #[derive(Debug, PartialEq)]
            #[from(json)]
            struct Min {
                #[min(value = 6)]
                field: $typ,
            }

            #[test]
            fn min_valid() {
                assert_eq!(
                    Min::from_json("{\"field\": 7}".as_bytes()).unwrap(),
                    Min { field: 7 }
                );

                assert_eq!(
                    Min::from_json_lang("{\"field\": 7}".as_bytes(), "en").unwrap(),
                    Min { field: 7 }
                );

                assert_eq!(
                    Min::from_json_stack_errs("{\"field\": 7}".as_bytes()).unwrap(),
                    Min { field: 7 }
                );

                assert_eq!(
                    Min::from_json_stack_errs_lang("{\"field\": 7}".as_bytes(), "en").unwrap(),
                    Min { field: 7 }
                );
            }

            #[test]
            fn min_invalid() {
                assert_eq!(
                    Min::from_json(r#"{"field": 5}"#.as_bytes()).unwrap_err(),
                    from::Err::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        "number must be greater than or equal to 6",
                    )
                );

                assert_eq!(
                    Min::from_json_lang(r#"{"field": 5}"#.as_bytes(), "en").unwrap_err(),
                    from::Err::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        "number must be greater than or equal to 6",
                    )
                );

                assert_eq!(
                    Min::from_json_stack_errs(r#"{"field": 5}"#.as_bytes()).unwrap_err(),
                    from::Errs::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        "number must be greater than or equal to 6",
                    )
                );

                assert_eq!(
                    Min::from_json_stack_errs_lang(r#"{"field": 5}"#.as_bytes(), "en").unwrap_err(),
                    from::Errs::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        "number must be greater than or equal to 6",
                    )
                );
            }

            #[derive(Debug, PartialEq)]
            #[from(json)]
            struct MinOverwriteMsg {
                #[min(value = 6, msgs{en="too small", ar="صغير جدا"})]
                field: $typ,
            }

            #[test]
            fn min_overwrite_msg_valid() {
                assert_eq!(
                    MinOverwriteMsg::from_json("{\"field\": 7}".as_bytes()).unwrap(),
                    MinOverwriteMsg { field: 7 }
                );

                assert_eq!(
                    MinOverwriteMsg::from_json_lang("{\"field\": 7}".as_bytes(), "en").unwrap(),
                    MinOverwriteMsg { field: 7 }
                );

                assert_eq!(
                    MinOverwriteMsg::from_json_stack_errs("{\"field\": 7}".as_bytes()).unwrap(),
                    MinOverwriteMsg { field: 7 }
                );

                assert_eq!(
                    MinOverwriteMsg::from_json_stack_errs_lang("{\"field\": 7}".as_bytes(), "en")
                        .unwrap(),
                    MinOverwriteMsg { field: 7 }
                );
            }

            #[test]
            fn min_overwrite_msg_invalid() {
                assert_eq!(
                    MinOverwriteMsg::from_json(r#"{"field": 5}"#.as_bytes()).unwrap_err(),
                    from::Err::new_validation_err(From::from("field"), Vec::new(), "too small",)
                );

                assert_eq!(
                    MinOverwriteMsg::from_json_lang(r#"{"field": 5}"#.as_bytes(), "ar")
                        .unwrap_err(),
                    from::Err::new_validation_err(From::from("field"), Vec::new(), "صغير جدا",)
                );

                assert_eq!(
                    MinOverwriteMsg::from_json_stack_errs(r#"{"field": 5}"#.as_bytes())
                        .unwrap_err(),
                    from::Errs::new_validation_err(From::from("field"), Vec::new(), "too small",)
                );

                assert_eq!(
                    MinOverwriteMsg::from_json_stack_errs_lang(r#"{"field": 5}"#.as_bytes(), "en")
                        .unwrap_err(),
                    from::Errs::new_validation_err(From::from("field"), Vec::new(), "too small",)
                );
            }

            #[derive(Debug, PartialEq)]
            #[from(json)]
            #[dflt_lang = "ar"]
            struct MinOverwriteMsgDfltLang {
                #[min(value = 6, msgs{en="too small", ar="صغير جدا"})]
                field: $typ,
            }

            #[test]
            fn min_overwrite_msg_dflt_lang_valid() {
                assert_eq!(
                    MinOverwriteMsgDfltLang::from_json("{\"field\": 7}".as_bytes()).unwrap(),
                    MinOverwriteMsgDfltLang { field: 7 }
                );

                assert_eq!(
                    MinOverwriteMsgDfltLang::from_json_lang("{\"field\": 7}".as_bytes(), "en")
                        .unwrap(),
                    MinOverwriteMsgDfltLang { field: 7 }
                );

                assert_eq!(
                    MinOverwriteMsgDfltLang::from_json_stack_errs("{\"field\": 7}".as_bytes())
                        .unwrap(),
                    MinOverwriteMsgDfltLang { field: 7 }
                );

                assert_eq!(
                    MinOverwriteMsgDfltLang::from_json_stack_errs_lang(
                        "{\"field\": 7}".as_bytes(),
                        "en"
                    )
                    .unwrap(),
                    MinOverwriteMsgDfltLang { field: 7 }
                );
            }

            #[test]
            fn min_overwrite_msg_dflt_lang_invalid() {
                assert_eq!(
                    MinOverwriteMsgDfltLang::from_json(r#"{"field": 5}"#.as_bytes()).unwrap_err(),
                    from::Err::new_validation_err(From::from("field"), Vec::new(), "صغير جدا",)
                );

                assert_eq!(
                    MinOverwriteMsgDfltLang::from_json_lang(r#"{"field": 5}"#.as_bytes(), "en")
                        .unwrap_err(),
                    from::Err::new_validation_err(From::from("field"), Vec::new(), "too small",)
                );

                assert_eq!(
                    MinOverwriteMsgDfltLang::from_json_stack_errs(r#"{"field": 5}"#.as_bytes())
                        .unwrap_err(),
                    from::Errs::new_validation_err(From::from("field"), Vec::new(), "صغير جدا",)
                );

                assert_eq!(
                    MinOverwriteMsgDfltLang::from_json_stack_errs_lang(
                        r#"{"field": 5}"#.as_bytes(),
                        "ar"
                    )
                    .unwrap_err(),
                    from::Errs::new_validation_err(From::from("field"), Vec::new(), "صغير جدا",)
                );
            }

            #[derive(Debug, PartialEq)]
            #[from(json)]
            struct Enum {
                #[r#enum(values = [5, 7])]
                field: $typ,
            }

            #[test]
            fn enum_valid() {
                assert_eq!(
                    Enum::from_json("{\"field\": 7}".as_bytes()).unwrap(),
                    Enum { field: 7 }
                );

                assert_eq!(
                    Enum::from_json_lang("{\"field\": 5}".as_bytes(), "en").unwrap(),
                    Enum { field: 5 }
                );

                assert_eq!(
                    Enum::from_json_stack_errs("{\"field\": 5}".as_bytes()).unwrap(),
                    Enum { field: 5 }
                );

                assert_eq!(
                    Enum::from_json_stack_errs_lang("{\"field\": 7}".as_bytes(), "en").unwrap(),
                    Enum { field: 7 }
                );
            }

            #[test]
            fn enum_invalid() {
                assert_eq!(
                    Enum::from_json(r#"{"field": 6}"#.as_bytes()).unwrap_err(),
                    from::Err::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        "number must be one of: [5, 7]",
                    )
                );

                assert_eq!(
                    Enum::from_json_lang(r#"{"field": 6}"#.as_bytes(), "en").unwrap_err(),
                    from::Err::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        "number must be one of: [5, 7]",
                    )
                );

                assert_eq!(
                    Enum::from_json_stack_errs(r#"{"field": 9}"#.as_bytes()).unwrap_err(),
                    from::Errs::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        "number must be one of: [5, 7]",
                    )
                );

                assert_eq!(
                    Enum::from_json_stack_errs_lang(r#"{"field": 8}"#.as_bytes(), "en")
                        .unwrap_err(),
                    from::Errs::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        "number must be one of: [5, 7]",
                    )
                );
            }

            #[derive(Debug, PartialEq)]
            #[from(json)]
            struct EnumOverwriteMsg {
                #[r#enum(values = [5, 7], msgs{en="invalid number", ar="عدد غير صالح"})]
                field: $typ,
            }

            #[test]
            fn enum_overwrite_msg_valid() {
                assert_eq!(
                    EnumOverwriteMsg::from_json("{\"field\": 5}".as_bytes()).unwrap(),
                    EnumOverwriteMsg { field: 5 }
                );

                assert_eq!(
                    EnumOverwriteMsg::from_json_lang("{\"field\": 7}".as_bytes(), "en").unwrap(),
                    EnumOverwriteMsg { field: 7 }
                );

                assert_eq!(
                    EnumOverwriteMsg::from_json_stack_errs("{\"field\": 5}".as_bytes()).unwrap(),
                    EnumOverwriteMsg { field: 5 }
                );

                assert_eq!(
                    EnumOverwriteMsg::from_json_stack_errs_lang("{\"field\": 7}".as_bytes(), "en")
                        .unwrap(),
                    EnumOverwriteMsg { field: 7 }
                );
            }

            #[test]
            fn enum_overwrite_msg_invalid() {
                assert_eq!(
                    EnumOverwriteMsg::from_json(r#"{"field": 8}"#.as_bytes()).unwrap_err(),
                    from::Err::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        "invalid number",
                    )
                );

                assert_eq!(
                    EnumOverwriteMsg::from_json_lang(r#"{"field": 9}"#.as_bytes(), "ar")
                        .unwrap_err(),
                    from::Err::new_validation_err(From::from("field"), Vec::new(), "عدد غير صالح",)
                );

                assert_eq!(
                    EnumOverwriteMsg::from_json_stack_errs(r#"{"field": 15}"#.as_bytes())
                        .unwrap_err(),
                    from::Errs::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        "invalid number",
                    )
                );

                assert_eq!(
                    EnumOverwriteMsg::from_json_stack_errs_lang(
                        r#"{"field": 20}"#.as_bytes(),
                        "ar"
                    )
                    .unwrap_err(),
                    from::Errs::new_validation_err(From::from("field"), Vec::new(), "عدد غير صالح",)
                );
            }

            #[derive(Debug, PartialEq)]
            #[from(json)]
            #[dflt_lang = "ar"]
            struct EnumOverwriteMsgDfltLang {
                #[r#enum(values = [5, 7], msgs{en="invalid number", ar="عدد غير صالح"})]
                field: $typ,
            }

            #[test]
            fn enum_overwrite_msg_dflt_lang_valid() {
                assert_eq!(
                    EnumOverwriteMsgDfltLang::from_json("{\"field\": 7}".as_bytes()).unwrap(),
                    EnumOverwriteMsgDfltLang { field: 7 }
                );

                assert_eq!(
                    EnumOverwriteMsgDfltLang::from_json_lang("{\"field\": 5}".as_bytes(), "en")
                        .unwrap(),
                    EnumOverwriteMsgDfltLang { field: 5 }
                );

                assert_eq!(
                    EnumOverwriteMsgDfltLang::from_json_stack_errs("{\"field\": 5}".as_bytes())
                        .unwrap(),
                    EnumOverwriteMsgDfltLang { field: 5 }
                );

                assert_eq!(
                    EnumOverwriteMsgDfltLang::from_json_stack_errs_lang(
                        "{\"field\": 5}".as_bytes(),
                        "en"
                    )
                    .unwrap(),
                    EnumOverwriteMsgDfltLang { field: 5 }
                );
            }

            #[test]
            fn enum_overwrite_msg_dflt_lang_invalid() {
                assert_eq!(
                    EnumOverwriteMsgDfltLang::from_json(r#"{"field": 8}"#.as_bytes()).unwrap_err(),
                    from::Err::new_validation_err(From::from("field"), Vec::new(), "عدد غير صالح",)
                );

                assert_eq!(
                    EnumOverwriteMsgDfltLang::from_json_lang(r#"{"field": 75}"#.as_bytes(), "en")
                        .unwrap_err(),
                    from::Err::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        "invalid number",
                    )
                );

                assert_eq!(
                    EnumOverwriteMsgDfltLang::from_json_stack_errs(r#"{"field": 53}"#.as_bytes())
                        .unwrap_err(),
                    from::Errs::new_validation_err(From::from("field"), Vec::new(), "عدد غير صالح",)
                );

                assert_eq!(
                    EnumOverwriteMsgDfltLang::from_json_stack_errs_lang(
                        r#"{"field": 25}"#.as_bytes(),
                        "en"
                    )
                    .unwrap_err(),
                    from::Errs::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        "invalid number",
                    )
                );
            }

            #[derive(Debug, PartialEq)]
            #[from(json)]
            struct Required {
                field: $typ,
            }

            #[test]
            fn required() {
                assert_eq!(
                    Required::from_json(r#"{"field1": 5}"#.as_bytes()).unwrap_err(),
                    from::Err::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        "required field",
                    )
                );

                assert_eq!(
                    Required::from_json_lang(r#"{"field2": 5}"#.as_bytes(), "en").unwrap_err(),
                    from::Err::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        "required field",
                    )
                );

                assert_eq!(
                    Required::from_json_stack_errs(r#"{"field4": 5}"#.as_bytes()).unwrap_err(),
                    from::Errs::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        "required field",
                    )
                );

                assert_eq!(
                    Required::from_json_stack_errs_lang(r#"{"field7": 5}"#.as_bytes(), "en")
                        .unwrap_err(),
                    from::Errs::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        "required field",
                    )
                );
            }

            #[derive(Debug, PartialEq)]
            #[from(json)]
            struct RequiredOverwriteMsg {
                #[required_msgs{en = "value is required", ar="الحقل مطلوب"}]
                field: $typ,
            }

            #[test]
            fn required_overwrite_msg() {
                assert_eq!(
                    RequiredOverwriteMsg::from_json(r#"{"field7": 5}"#.as_bytes()).unwrap_err(),
                    from::Err::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        "value is required",
                    )
                );

                assert_eq!(
                    RequiredOverwriteMsg::from_json_lang(r#"{"field9": 5}"#.as_bytes(), "en")
                        .unwrap_err(),
                    from::Err::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        "value is required",
                    )
                );

                assert_eq!(
                    RequiredOverwriteMsg::from_json_stack_errs(r#"{}"#.as_bytes()).unwrap_err(),
                    from::Errs::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        "value is required",
                    )
                );

                assert_eq!(
                    RequiredOverwriteMsg::from_json_stack_errs_lang(r#"{  }"#.as_bytes(), "ar")
                        .unwrap_err(),
                    from::Errs::new_validation_err(From::from("field"), Vec::new(), "الحقل مطلوب",)
                );
            }

            #[derive(Debug, PartialEq)]
            #[from(json)]
            #[dflt_lang = "ar"]
            struct RequiredOverwriteMsgDfltLang {
                #[required_msgs{en = "value is required", ar="الحقل مطلوب"}]
                field: $typ,
            }

            #[test]
            fn required_overwrite_msg_dflt_lang() {
                assert_eq!(
                    RequiredOverwriteMsgDfltLang::from_json(r#"{"field7": 5}"#.as_bytes())
                        .unwrap_err(),
                    from::Err::new_validation_err(From::from("field"), Vec::new(), "الحقل مطلوب",)
                );

                assert_eq!(
                    RequiredOverwriteMsgDfltLang::from_json_lang(
                        r#"{"field9": 5}"#.as_bytes(),
                        "en"
                    )
                    .unwrap_err(),
                    from::Err::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        "value is required",
                    )
                );

                assert_eq!(
                    RequiredOverwriteMsgDfltLang::from_json_stack_errs(r#"{}"#.as_bytes())
                        .unwrap_err(),
                    from::Errs::new_validation_err(From::from("field"), Vec::new(), "الحقل مطلوب",)
                );

                assert_eq!(
                    RequiredOverwriteMsgDfltLang::from_json_stack_errs_lang(
                        r#"{  }"#.as_bytes(),
                        "ar"
                    )
                    .unwrap_err(),
                    from::Errs::new_validation_err(From::from("field"), Vec::new(), "الحقل مطلوب",)
                );
            }

            //
            //
            //

            #[derive(Debug, PartialEq)]
            #[from(json)]
            struct TooLarge {
                field: $typ,
            }

            #[test]
            fn too_large() {
                assert_eq!(
                    TooLarge::from_json(json_too_large($typ::MAX.to_string()).as_bytes())
                        .unwrap_err(),
                    from::Err::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        format!("number is too large to fit in '{}' type", $typ_str),
                    )
                );

                assert_eq!(
                    TooLarge::from_json_lang(
                        json_too_large($typ::MAX.to_string()).as_bytes(),
                        "en"
                    )
                    .unwrap_err(),
                    from::Err::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        format!("number is too large to fit in '{}' type", $typ_str),
                    )
                );

                assert_eq!(
                    TooLarge::from_json_stack_errs(
                        json_too_large($typ::MAX.to_string()).as_bytes()
                    )
                    .unwrap_err(),
                    from::Errs::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        format!("number is too large to fit in '{}' type", $typ_str),
                    )
                );

                assert_eq!(
                    TooLarge::from_json_stack_errs_lang(
                        json_too_large($typ::MAX.to_string()).as_bytes(),
                        "en"
                    )
                    .unwrap_err(),
                    from::Errs::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        format!("number is too large to fit in '{}' type", $typ_str),
                    )
                );
            }

            #[derive(Debug, PartialEq)]
            #[from(json)]
            struct TooLargeOverwriteMsg {
                #[too_large_msgs{en="cannot fit", ar="لا يتناسب"}]
                field: $typ,
            }

            #[test]
            fn too_large_overwrite_msg() {
                assert_eq!(
                    TooLargeOverwriteMsg::from_json(
                        json_too_large($typ::MAX.to_string()).as_bytes()
                    )
                    .unwrap_err(),
                    from::Err::new_validation_err(From::from("field"), Vec::new(), "cannot fit",)
                );

                assert_eq!(
                    TooLargeOverwriteMsg::from_json_lang(
                        json_too_large($typ::MAX.to_string()).as_bytes(),
                        "ar"
                    )
                    .unwrap_err(),
                    from::Err::new_validation_err(From::from("field"), Vec::new(), "لا يتناسب",)
                );

                assert_eq!(
                    TooLargeOverwriteMsg::from_json_stack_errs(
                        json_too_large($typ::MAX.to_string()).as_bytes()
                    )
                    .unwrap_err(),
                    from::Errs::new_validation_err(From::from("field"), Vec::new(), "cannot fit",)
                );

                assert_eq!(
                    TooLargeOverwriteMsg::from_json_stack_errs_lang(
                        json_too_large($typ::MAX.to_string()).as_bytes(),
                        "en"
                    )
                    .unwrap_err(),
                    from::Errs::new_validation_err(From::from("field"), Vec::new(), "cannot fit",)
                );
            }

            #[derive(Debug, PartialEq)]
            #[from(json)]
            #[dflt_lang = "ar"]
            struct TooLargeOverwriteMsgDfltLang {
                #[too_large_msgs{en="cannot fit", ar="لا يتناسب"}]
                field: $typ,
            }

            #[test]
            fn too_large_overwrite_msg_dflt_lang() {
                assert_eq!(
                    TooLargeOverwriteMsgDfltLang::from_json(
                        json_too_large($typ::MAX.to_string()).as_bytes()
                    )
                    .unwrap_err(),
                    from::Err::new_validation_err(From::from("field"), Vec::new(), "لا يتناسب",)
                );

                assert_eq!(
                    TooLargeOverwriteMsgDfltLang::from_json_lang(
                        json_too_large($typ::MAX.to_string()).as_bytes(),
                        "ar"
                    )
                    .unwrap_err(),
                    from::Err::new_validation_err(From::from("field"), Vec::new(), "لا يتناسب",)
                );

                assert_eq!(
                    TooLargeOverwriteMsgDfltLang::from_json_stack_errs(
                        json_too_large($typ::MAX.to_string()).as_bytes()
                    )
                    .unwrap_err(),
                    from::Errs::new_validation_err(From::from("field"), Vec::new(), "لا يتناسب",)
                );

                assert_eq!(
                    TooLargeOverwriteMsgDfltLang::from_json_stack_errs_lang(
                        json_too_large($typ::MAX.to_string()).as_bytes(),
                        "en"
                    )
                    .unwrap_err(),
                    from::Errs::new_validation_err(From::from("field"), Vec::new(), "cannot fit",)
                );
            }

            //
            //
            //

            #[derive(Debug, PartialEq)]
            #[from(json)]
            struct TooSmall {
                field: $typ,
            }

            #[test]
            fn too_small() {
                assert_eq!(
                    TooSmall::from_json(json_too_small($typ::MIN.to_string()).as_bytes())
                        .unwrap_err(),
                    from::Err::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        format!("number is too small to fit in '{}' type", $typ_str),
                    )
                );

                assert_eq!(
                    TooSmall::from_json_lang(
                        json_too_small($typ::MIN.to_string()).as_bytes(),
                        "en"
                    )
                    .unwrap_err(),
                    from::Err::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        format!("number is too small to fit in '{}' type", $typ_str),
                    )
                );

                assert_eq!(
                    TooSmall::from_json_stack_errs(
                        json_too_small($typ::MIN.to_string()).as_bytes()
                    )
                    .unwrap_err(),
                    from::Errs::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        format!("number is too small to fit in '{}' type", $typ_str),
                    )
                );

                assert_eq!(
                    TooSmall::from_json_stack_errs_lang(
                        json_too_small($typ::MIN.to_string()).as_bytes(),
                        "en"
                    )
                    .unwrap_err(),
                    from::Errs::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        format!("number is too small to fit in '{}' type", $typ_str),
                    )
                );
            }

            #[derive(Debug, PartialEq)]
            #[from(json)]
            struct TooSmallOverwriteMsg {
                #[too_small_msgs{en="cannot fit", ar="لا يتناسب"}]
                field: $typ,
            }

            #[test]
            fn too_small_overwrite_msg() {
                assert_eq!(
                    TooSmallOverwriteMsg::from_json(
                        json_too_small($typ::MIN.to_string()).as_bytes()
                    )
                    .unwrap_err(),
                    from::Err::new_validation_err(From::from("field"), Vec::new(), "cannot fit",)
                );

                assert_eq!(
                    TooSmallOverwriteMsg::from_json_lang(
                        json_too_small($typ::MIN.to_string()).as_bytes(),
                        "ar"
                    )
                    .unwrap_err(),
                    from::Err::new_validation_err(From::from("field"), Vec::new(), "لا يتناسب",)
                );

                assert_eq!(
                    TooSmallOverwriteMsg::from_json_stack_errs(
                        json_too_small($typ::MIN.to_string()).as_bytes()
                    )
                    .unwrap_err(),
                    from::Errs::new_validation_err(From::from("field"), Vec::new(), "cannot fit",)
                );

                assert_eq!(
                    TooSmallOverwriteMsg::from_json_stack_errs_lang(
                        json_too_small($typ::MIN.to_string()).as_bytes(),
                        "en"
                    )
                    .unwrap_err(),
                    from::Errs::new_validation_err(From::from("field"), Vec::new(), "cannot fit",)
                );
            }

            #[derive(Debug, PartialEq)]
            #[from(json)]
            #[dflt_lang = "ar"]
            struct TooSmallOverwriteMsgDfltLang {
                #[too_small_msgs{en="cannot fit", ar="لا يتناسب"}]
                field: $typ,
            }

            #[test]
            fn too_small_overwrite_msg_dflt_lang() {
                assert_eq!(
                    TooSmallOverwriteMsgDfltLang::from_json(
                        json_too_small($typ::MIN.to_string()).as_bytes()
                    )
                    .unwrap_err(),
                    from::Err::new_validation_err(From::from("field"), Vec::new(), "لا يتناسب",)
                );

                assert_eq!(
                    TooSmallOverwriteMsgDfltLang::from_json_lang(
                        json_too_small($typ::MIN.to_string()).as_bytes(),
                        "ar"
                    )
                    .unwrap_err(),
                    from::Err::new_validation_err(From::from("field"), Vec::new(), "لا يتناسب",)
                );

                assert_eq!(
                    TooSmallOverwriteMsgDfltLang::from_json_stack_errs(
                        json_too_small($typ::MIN.to_string()).as_bytes()
                    )
                    .unwrap_err(),
                    from::Errs::new_validation_err(From::from("field"), Vec::new(), "لا يتناسب",)
                );

                assert_eq!(
                    TooSmallOverwriteMsgDfltLang::from_json_stack_errs_lang(
                        json_too_small($typ::MIN.to_string()).as_bytes(),
                        "en"
                    )
                    .unwrap_err(),
                    from::Errs::new_validation_err(From::from("field"), Vec::new(), "cannot fit",)
                );
            }

            //
            //
            //
            //

            #[derive(Debug, PartialEq)]
            #[from(json)]
            struct TypeMismatch {
                field: $typ,
            }

            #[test]
            fn type_mismatch() {
                assert_eq!(
                    TypeMismatch::from_json(r#"{"field": ""}"#.as_bytes()).unwrap_err(),
                    from::Err::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        format!("expected: {}, found: string", $typ_str),
                    )
                );

                assert_eq!(
                    TypeMismatch::from_json_lang(r#"{"field": "as"}"#.as_bytes(), "en")
                        .unwrap_err(),
                    from::Err::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        format!("expected: {}, found: string", $typ_str),
                    )
                );

                assert_eq!(
                    TypeMismatch::from_json_stack_errs(r#"{"field": "25"}"#.as_bytes())
                        .unwrap_err(),
                    from::Errs::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        format!("expected: {}, found: string", $typ_str),
                    )
                );

                assert_eq!(
                    TypeMismatch::from_json_stack_errs_lang(
                        r#"{"field": "asd788q"}"#.as_bytes(),
                        "en"
                    )
                    .unwrap_err(),
                    from::Errs::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        format!("expected: {}, found: string", $typ_str),
                    )
                );

                assert_eq!(
                    TypeMismatch::from_json(r#"{"field": true}"#.as_bytes()).unwrap_err(),
                    from::Err::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        format!("expected: {}, found: boolean", $typ_str),
                    )
                );

                assert_eq!(
                    TypeMismatch::from_json_lang(r#"{"field": false}"#.as_bytes(), "en")
                        .unwrap_err(),
                    from::Err::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        format!("expected: {}, found: boolean", $typ_str),
                    )
                );

                assert_eq!(
                    TypeMismatch::from_json_stack_errs(r#"{"field": true}"#.as_bytes())
                        .unwrap_err(),
                    from::Errs::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        format!("expected: {}, found: boolean", $typ_str),
                    )
                );

                assert_eq!(
                    TypeMismatch::from_json_stack_errs_lang(r#"{"field": true}"#.as_bytes(), "en")
                        .unwrap_err(),
                    from::Errs::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        format!("expected: {}, found: boolean", $typ_str),
                    )
                );

                assert_eq!(
                    TypeMismatch::from_json(r#"{"field": {this will be ignored}}"#.as_bytes())
                        .unwrap_err(),
                    from::Err::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        format!("expected: {}, found: object", $typ_str),
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
                        format!("expected: {}, found: object", $typ_str),
                    )
                );

                assert_eq!(
                    TypeMismatch::from_json_stack_errs(
                        r#"{"field": {because it does not care}}"#.as_bytes()
                    )
                    .unwrap_err(),
                    from::Errs::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        format!("expected: {}, found: object", $typ_str),
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
                        format!("expected: {}, found: object", $typ_str),
                    )
                );

                assert_eq!(
                    TypeMismatch::from_json(r#"{"field": [this will be ignored]}"#.as_bytes())
                        .unwrap_err(),
                    from::Err::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        format!("expected: {}, found: array", $typ_str),
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
                        format!("expected: {}, found: array", $typ_str),
                    )
                );

                assert_eq!(
                    TypeMismatch::from_json_stack_errs(
                        r#"{"field": [because it does not care]}"#.as_bytes()
                    )
                    .unwrap_err(),
                    from::Errs::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        format!("expected: {}, found: array", $typ_str),
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
                        format!("expected: {}, found: array", $typ_str),
                    )
                );
            }

            #[derive(Debug, PartialEq)]
            #[from(json)]
            struct TypeMismatchOverwriteMsg {
                #[type_mismatch_msgs{en="cannot fit", ar="لا يتناسب"}]
                field: $typ,
            }

            #[test]
            fn type_mismatch_overwrite_msg() {
                assert_eq!(
                    TypeMismatchOverwriteMsg::from_json(r#"{"field": false}"#.as_bytes())
                        .unwrap_err(),
                    from::Err::new_validation_err(From::from("field"), Vec::new(), "cannot fit",)
                );

                assert_eq!(
                    TypeMismatchOverwriteMsg::from_json_lang(r#"{"field": "25"}"#.as_bytes(), "ar")
                        .unwrap_err(),
                    from::Err::new_validation_err(From::from("field"), Vec::new(), "لا يتناسب",)
                );

                assert_eq!(
                    TypeMismatchOverwriteMsg::from_json_stack_errs(r#"{"field": {}}"#.as_bytes())
                        .unwrap_err(),
                    from::Errs::new_validation_err(From::from("field"), Vec::new(), "cannot fit",)
                );

                assert_eq!(
                    TypeMismatchOverwriteMsg::from_json_stack_errs_lang(
                        r#"{"field": [asd[asd[]asd]asd]}"#.as_bytes(),
                        "en"
                    )
                    .unwrap_err(),
                    from::Errs::new_validation_err(From::from("field"), Vec::new(), "cannot fit",)
                );
            }

            #[derive(Debug, PartialEq)]
            #[from(json)]
            #[dflt_lang = "ar"]
            struct TypeMismatchOverwriteMsgDfltLang {
                #[type_mismatch_msgs{en="cannot fit", ar="لا يتناسب"}]
                field: $typ,
            }

            #[test]
            fn type_mismatch_overwrite_msg_dflt_lang() {
                assert_eq!(
                    TypeMismatchOverwriteMsgDfltLang::from_json(r#"{"field": false}"#.as_bytes())
                        .unwrap_err(),
                    from::Err::new_validation_err(From::from("field"), Vec::new(), "لا يتناسب",)
                );

                assert_eq!(
                    TypeMismatchOverwriteMsgDfltLang::from_json_lang(
                        r#"{"field": "asdqw"}"#.as_bytes(),
                        "ar"
                    )
                    .unwrap_err(),
                    from::Err::new_validation_err(From::from("field"), Vec::new(), "لا يتناسب",)
                );

                assert_eq!(
                    TypeMismatchOverwriteMsgDfltLang::from_json_stack_errs(
                        r#"{"field": {}}"#.as_bytes()
                    )
                    .unwrap_err(),
                    from::Errs::new_validation_err(From::from("field"), Vec::new(), "لا يتناسب",)
                );

                assert_eq!(
                    TypeMismatchOverwriteMsgDfltLang::from_json_stack_errs_lang(
                        r#"{"field": [asd[asd[]asd]asd]}"#.as_bytes(),
                        "en"
                    )
                    .unwrap_err(),
                    from::Errs::new_validation_err(From::from("field"), Vec::new(), "cannot fit",)
                );
            }

            #[derive(Debug, PartialEq)]
            #[from(json)]
            struct NotNull {
                field: $typ,
            }

            #[test]
            fn not_null() {
                assert_eq!(
                    NotNull::from_json(r#"{"field": null}"#.as_bytes()).unwrap_err(),
                    from::Err::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        format!("expected: {}, found: null", $typ_str)
                    )
                );

                assert_eq!(
                    NotNull::from_json_lang(r#"{"field": null}"#.as_bytes(), "ar").unwrap_err(),
                    from::Err::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        format!("expected: {}, found: null", $typ_str),
                    )
                );

                assert_eq!(
                    NotNull::from_json_stack_errs(r#"{"field": null}"#.as_bytes()).unwrap_err(),
                    from::Errs::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        format!("expected: {}, found: null", $typ_str),
                    )
                );

                assert_eq!(
                    NotNull::from_json_stack_errs_lang(r#"{"field": null}"#.as_bytes(), "en")
                        .unwrap_err(),
                    from::Errs::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        format!("expected: {}, found: null", $typ_str),
                    )
                );
            }

            #[derive(Debug, PartialEq)]
            #[from(json)]
            struct NotNullOverwriteMsg {
                #[not_null_msgs{en="null is not allowed", ar="القيمة الخالية غير مسموحة"}]
                field: $typ,
            }

            #[test]
            fn not_null_overwrite_msg() {
                assert_eq!(
                    NotNullOverwriteMsg::from_json(r#"{"field": null}"#.as_bytes()).unwrap_err(),
                    from::Err::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        "null is not allowed",
                    )
                );

                assert_eq!(
                    NotNullOverwriteMsg::from_json_lang(r#"{"field": null}"#.as_bytes(), "ar")
                        .unwrap_err(),
                    from::Err::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        "القيمة الخالية غير مسموحة",
                    )
                );

                assert_eq!(
                    NotNullOverwriteMsg::from_json_stack_errs(r#"{"field": null}"#.as_bytes())
                        .unwrap_err(),
                    from::Errs::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        "null is not allowed",
                    )
                );

                assert_eq!(
                    NotNullOverwriteMsg::from_json_stack_errs_lang(
                        r#"{"field": null}"#.as_bytes(),
                        "en"
                    )
                    .unwrap_err(),
                    from::Errs::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        "null is not allowed",
                    )
                );
            }

            #[derive(Debug, PartialEq)]
            #[from(json)]
            #[dflt_lang = "ar"]
            struct NotNullOverwriteMsgDfltLang {
                #[not_null_msgs{en="null is not allowed", ar="القيمة الخالية غير مسموحة"}]
                field: $typ,
            }

            #[test]
            fn not_null_overwrite_msg_dflt_lang() {
                assert_eq!(
                    NotNullOverwriteMsgDfltLang::from_json(r#"{"field": null}"#.as_bytes())
                        .unwrap_err(),
                    from::Err::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        "القيمة الخالية غير مسموحة",
                    )
                );

                assert_eq!(
                    NotNullOverwriteMsgDfltLang::from_json_lang(
                        r#"{"field": null}"#.as_bytes(),
                        "ar"
                    )
                    .unwrap_err(),
                    from::Err::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        "القيمة الخالية غير مسموحة",
                    )
                );

                assert_eq!(
                    NotNullOverwriteMsgDfltLang::from_json_stack_errs(
                        r#"{"field": null}"#.as_bytes()
                    )
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
                    from::Errs::new_validation_err(
                        From::from("field"),
                        Vec::new(),
                        "null is not allowed",
                    )
                );
            }

            #[derive(Debug, PartialEq)]
            #[from(json)]
            struct NullValue {
                field: Null<$typ>,
            }

            #[test]
            fn null() {
                assert_eq!(
                    NullValue::from_json(r#"{"field": null}"#.as_bytes()).unwrap(),
                    NullValue { field: Null::Null }
                );

                assert_eq!(
                    NullValue::from_json(r#"{"field": 7}"#.as_bytes()).unwrap(),
                    NullValue {
                        field: Null::Some(7),
                    }
                );

                assert_eq!(
                    NullValue::from_json_lang(r#"{"field": null}"#.as_bytes(), "en").unwrap(),
                    NullValue { field: Null::Null }
                );

                assert_eq!(
                    NullValue::from_json_lang(r#"{"field": 49}"#.as_bytes(), "en").unwrap(),
                    NullValue {
                        field: Null::Some(49),
                    }
                );

                assert_eq!(
                    NullValue::from_json_stack_errs(r#"{"field": null}"#.as_bytes()).unwrap(),
                    NullValue { field: Null::Null }
                );

                assert_eq!(
                    NullValue::from_json_stack_errs(r#"{"field": 110}"#.as_bytes()).unwrap(),
                    NullValue {
                        field: Null::Some(110),
                    }
                );

                assert_eq!(
                    NullValue::from_json_stack_errs_lang(r#"{"field": null}"#.as_bytes(), "ar")
                        .unwrap(),
                    NullValue { field: Null::Null }
                );

                assert_eq!(
                    NullValue::from_json_stack_errs_lang(r#"{"field": 48}"#.as_bytes(), "ar")
                        .unwrap(),
                    NullValue {
                        field: Null::Some(48),
                    }
                );
            }

            #[derive(Debug, PartialEq)]
            #[from(json)]
            struct OptionValue {
                field: Option<$typ>,
            }

            #[test]
            fn option() {
                assert_eq!(
                    OptionValue::from_json(r#"{"field7": null}"#.as_bytes()).unwrap(),
                    OptionValue { field: None }
                );

                assert_eq!(
                    OptionValue::from_json(r#"{"field": 77}"#.as_bytes()).unwrap(),
                    OptionValue { field: Some(77) }
                );

                assert_eq!(
                    OptionValue::from_json_lang(r#"{"fiel3d": null}"#.as_bytes(), "en").unwrap(),
                    OptionValue { field: None }
                );

                assert_eq!(
                    OptionValue::from_json_lang(r#"{"field": 88}"#.as_bytes(), "en").unwrap(),
                    OptionValue { field: Some(88) }
                );

                assert_eq!(
                    OptionValue::from_json_stack_errs(r#"{"fiel3d": null}"#.as_bytes()).unwrap(),
                    OptionValue { field: None }
                );

                assert_eq!(
                    OptionValue::from_json_stack_errs(r#"{"field": 13}"#.as_bytes()).unwrap(),
                    OptionValue { field: Some(13) }
                );

                assert_eq!(
                    OptionValue::from_json_stack_errs_lang(r#"{"fi7eld": null}"#.as_bytes(), "ar")
                        .unwrap(),
                    OptionValue { field: None }
                );

                assert_eq!(
                    OptionValue::from_json_stack_errs_lang(r#"{"field": 18}"#.as_bytes(), "ar")
                        .unwrap(),
                    OptionValue { field: Some(18) }
                );
            }

            #[derive(Debug, PartialEq)]
            #[from(json)]
            struct OptionNullValue {
                field: OptionNull<$typ>,
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
                    OptionNullValue::from_json(r#"{"field": 23}"#.as_bytes()).unwrap(),
                    OptionNullValue {
                        field: OptionNull::Some(23),
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
                    OptionNullValue::from_json_lang(r#"{"field": 27}"#.as_bytes(), "ar").unwrap(),
                    OptionNullValue {
                        field: OptionNull::Some(27),
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
                    OptionNullValue::from_json_stack_errs(r#"{"field": 32}"#.as_bytes()).unwrap(),
                    OptionNullValue {
                        field: OptionNull::Some(32),
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
                    OptionNullValue::from_json_stack_errs_lang(
                        r#"{"field": null}"#.as_bytes(),
                        "en"
                    )
                    .unwrap(),
                    OptionNullValue {
                        field: OptionNull::Null
                    }
                );

                assert_eq!(
                    OptionNullValue::from_json_stack_errs_lang(r#"{"field": 37}"#.as_bytes(), "en")
                        .unwrap(),
                    OptionNullValue {
                        field: OptionNull::Some(37),
                    }
                );
            }

            #[derive(Debug, PartialEq)]
            #[from(json)]
            struct DefaultValue {
                #[default = 10]
                field: $typ,
            }

            #[test]
            fn default() {
                assert_eq!(
                    DefaultValue::from_json(r#"{}"#.as_bytes()).unwrap(),
                    DefaultValue { field: 10 }
                );

                assert_eq!(
                    DefaultValue::from_json_lang(r#"{}"#.as_bytes(), "ar").unwrap(),
                    DefaultValue { field: 10 }
                );

                assert_eq!(
                    DefaultValue::from_json_stack_errs(r#"{}"#.as_bytes()).unwrap(),
                    DefaultValue { field: 10 }
                );

                assert_eq!(
                    DefaultValue::from_json_stack_errs_lang(r#"{}"#.as_bytes(), "en").unwrap(),
                    DefaultValue { field: 10 }
                );
            }

            #[derive(Debug, PartialEq)]
            #[from(json)]
            struct DefaultNullValue {
                #[default = null]
                field: Null<$typ>,
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
                field1: $typ,
                field2: $typ,
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

            struct Age;

            impl ::from::Validator<$typ> for Age {
                fn none(val: &$typ, path: &Path) -> Result<(), ValidationErr> {
                    if *val < 5 {
                        return Err(ValidationErr::new(From::from("age"), path.clone(), "young"));
                    };
                    Ok(())
                }
            }

            #[derive(Debug, PartialEq)]
            #[from(json)]
            struct CustomValidator {
                #[validators(Age)]
                age: $typ,
            }

            #[test]
            fn custom_validator() {
                assert_eq!(
                    CustomValidator::from_json(r#"{"age": 4}"#.as_bytes()).unwrap_err(),
                    from::Err::new_validation_err(From::from("age"), Vec::new(), "young",)
                );

                assert_eq!(
                    CustomValidator::from_json_lang(r#"{"age": 4}"#.as_bytes(), "ar").unwrap_err(),
                    from::Err::new_validation_err(From::from("age"), Vec::new(), "young",)
                );

                assert_eq!(
                    CustomValidator::from_json_stack_errs(r#"{"age": 4}"#.as_bytes()).unwrap_err(),
                    from::Errs::new_validation_err(From::from("age"), Vec::new(), "young",)
                );

                assert_eq!(
                    CustomValidator::from_json_stack_errs_lang(r#"{"age": 4}"#.as_bytes(), "en")
                        .unwrap_err(),
                    from::Errs::new_validation_err(From::from("age"), Vec::new(), "young",)
                );
            }
        }
    };
}

ints!(
    typ: i8,
    typ_str: "i8",
);

ints!(
    typ: i16,
    typ_str: "i16",
);

ints!(
    typ: i32,
    typ_str: "i32",
);

ints!(
    typ: i64,
    typ_str: "i64",
);

ints!(
    typ: i128,
    typ_str: "i128",
);

ints!(
    typ: isize,
    typ_str: "isize",
);

ints!(
    typ: u8,
    typ_str: "u8",
);

ints!(
    typ: u16,
    typ_str: "u16",
);

ints!(
    typ: u32,
    typ_str: "u32",
);

ints!(
    typ: u64,
    typ_str: "u64",
);

ints!(
    typ: u128,
    typ_str: "u128",
);

ints!(
    typ: usize,
    typ_str: "usize",
);
