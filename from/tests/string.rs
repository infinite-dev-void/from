use from::{from, FromJson, Null, OptionNull, Path, ValidationErr};

#[derive(Debug, PartialEq)]
#[from(json)]
struct Normal {
    field: String,
}

#[test]
fn normal() {
    assert_eq!(
        Normal::from_json("{\"field\": \"value\"}".as_bytes()).unwrap(),
        Normal {
            field: String::from("value"),
        }
    );

    assert_eq!(
        Normal::from_json_lang("{\"field\": \"value\"}".as_bytes(), "en").unwrap(),
        Normal {
            field: String::from("value"),
        }
    );

    assert_eq!(
        Normal::from_json_stack_errs("{\"field\": \"value\"}".as_bytes()).unwrap(),
        Normal {
            field: String::from("value"),
        }
    );

    assert_eq!(
        Normal::from_json_stack_errs_lang("{\"field\": \"value\"}".as_bytes(), "en").unwrap(),
        Normal {
            field: String::from("value"),
        }
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct Trim {
    #[trim]
    field: String,
}

#[test]
fn trim() {
    assert_eq!(
        Trim::from_json("{\"field\": \"  value \n\t\"}".as_bytes()).unwrap(),
        Trim {
            field: String::from("value"),
        }
    );

    assert_eq!(
        Trim::from_json_lang("{\"field\": \"  value \n\t\"}".as_bytes(), "en").unwrap(),
        Trim {
            field: String::from("value"),
        }
    );

    assert_eq!(
        Trim::from_json_stack_errs("{\"field\": \"  value \n\t\"}".as_bytes()).unwrap(),
        Trim {
            field: String::from("value"),
        }
    );

    assert_eq!(
        Trim::from_json_stack_errs_lang("{\"field\": \"  value \n\t\"}".as_bytes(), "en").unwrap(),
        Trim {
            field: String::from("value"),
        }
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct TrimEnd {
    #[trim_end]
    field: String,
}

#[test]
fn trim_end() {
    assert_eq!(
        TrimEnd::from_json("{\"field\": \"  value \n\t\"}".as_bytes()).unwrap(),
        TrimEnd {
            field: String::from("  value"),
        }
    );

    assert_eq!(
        TrimEnd::from_json_lang("{\"field\": \"  value \n\t\"}".as_bytes(), "en").unwrap(),
        TrimEnd {
            field: String::from("  value"),
        }
    );

    assert_eq!(
        TrimEnd::from_json_stack_errs("{\"field\": \"  value \n\t\"}".as_bytes()).unwrap(),
        TrimEnd {
            field: String::from("  value"),
        }
    );

    assert_eq!(
        TrimEnd::from_json_stack_errs_lang("{\"field\": \"  value \n\t\"}".as_bytes(), "en")
            .unwrap(),
        TrimEnd {
            field: String::from("  value"),
        }
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct TrimStart {
    #[trim_start]
    field: String,
}

#[test]
fn trim_start() {
    assert_eq!(
        TrimStart::from_json("{\"field\": \"  \n\tvalue \"}".as_bytes()).unwrap(),
        TrimStart {
            field: String::from("value "),
        }
    );

    assert_eq!(
        TrimStart::from_json_lang("{\"field\": \"  \n\tvalue \"}".as_bytes(), "en").unwrap(),
        TrimStart {
            field: String::from("value "),
        }
    );

    assert_eq!(
        TrimStart::from_json_stack_errs("{\"field\": \"  \n\tvalue \"}".as_bytes()).unwrap(),
        TrimStart {
            field: String::from("value "),
        }
    );

    assert_eq!(
        TrimStart::from_json_stack_errs_lang("{\"field\": \"  \n\tvalue \"}".as_bytes(), "en")
            .unwrap(),
        TrimStart {
            field: String::from("value "),
        }
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct Xss {
    #[sanitize_xss]
    field: String,
}

#[test]
fn sanitize_xss() {
    assert_eq!(
        Xss::from_json("{\"field\": \"<script>&where\\\"'\u{0000}</script>\"}".as_bytes()).unwrap(),
        Xss {
            field: String::from("&lt;script&gt;&amp;where&#34;&#39;\u{FFFD}&lt;/script&gt;"),
        }
    );

    assert_eq!(
        Xss::from_json_lang(
            "{\"field\": \"<script>&where\\\"'\u{0000}</script>\"}".as_bytes(),
            "ar"
        )
        .unwrap(),
        Xss {
            field: String::from("&lt;script&gt;&amp;where&#34;&#39;\u{FFFD}&lt;/script&gt;"),
        }
    );

    assert_eq!(
        Xss::from_json_stack_errs(
            "{\"field\": \"<script>&where\\\"'\u{0000}</script>\"}".as_bytes()
        )
        .unwrap(),
        Xss {
            field: String::from("&lt;script&gt;&amp;where&#34;&#39;\u{FFFD}&lt;/script&gt;"),
        }
    );

    assert_eq!(
        Xss::from_json_stack_errs_lang(
            "{\"field\": \"<script>&where\\\"'\u{0000}</script>\"}".as_bytes(),
            "fr",
        )
        .unwrap(),
        Xss {
            field: String::from("&lt;script&gt;&amp;where&#34;&#39;\u{FFFD}&lt;/script&gt;"),
        }
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct MaxLen {
    #[max_len(value = 4)]
    field: String,
}

#[test]
fn max_len_valid() {
    assert_eq!(
        MaxLen::from_json("{\"field\": \"val\"}".as_bytes()).unwrap(),
        MaxLen {
            field: String::from("val"),
        }
    );

    assert_eq!(
        MaxLen::from_json_lang("{\"field\": \"val\"}".as_bytes(), "en").unwrap(),
        MaxLen {
            field: String::from("val"),
        }
    );

    assert_eq!(
        MaxLen::from_json_stack_errs("{\"field\": \"val\"}".as_bytes()).unwrap(),
        MaxLen {
            field: String::from("val"),
        }
    );

    assert_eq!(
        MaxLen::from_json_stack_errs_lang("{\"field\": \"val\"}".as_bytes(), "en").unwrap(),
        MaxLen {
            field: String::from("val"),
        }
    );
}

#[test]
fn max_len_invalid() {
    assert_eq!(
        MaxLen::from_json(r#"{"field": "value"}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(
            From::from("field"),
            Vec::new(),
            "the string value must be no longer than 4 bytes",
        )
    );

    assert_eq!(
        MaxLen::from_json_lang(r#"{"field": "value"}"#.as_bytes(), "en").unwrap_err(),
        from::Err::new_validation_err(
            From::from("field"),
            Vec::new(),
            "the string value must be no longer than 4 bytes",
        )
    );

    assert_eq!(
        MaxLen::from_json_stack_errs(r#"{"field": "value"}"#.as_bytes()).unwrap_err(),
        from::Errs::new_validation_err(
            From::from("field"),
            Vec::new(),
            "the string value must be no longer than 4 bytes",
        )
    );

    assert_eq!(
        MaxLen::from_json_stack_errs_lang(r#"{"field": "value"}"#.as_bytes(), "en").unwrap_err(),
        from::Errs::new_validation_err(
            From::from("field"),
            Vec::new(),
            "the string value must be no longer than 4 bytes",
        )
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct MaxLenOverwriteMsg {
    #[max_len(value = 4, msgs{en="too long", ar="طويل جدا"})]
    field: String,
}

#[test]
fn max_len_overwrite_msg_valid() {
    assert_eq!(
        MaxLenOverwriteMsg::from_json("{\"field\": \"val\"}".as_bytes()).unwrap(),
        MaxLenOverwriteMsg {
            field: String::from("val"),
        }
    );

    assert_eq!(
        MaxLenOverwriteMsg::from_json_lang("{\"field\": \"val\"}".as_bytes(), "en").unwrap(),
        MaxLenOverwriteMsg {
            field: String::from("val"),
        }
    );

    assert_eq!(
        MaxLenOverwriteMsg::from_json_stack_errs("{\"field\": \"val\"}".as_bytes()).unwrap(),
        MaxLenOverwriteMsg {
            field: String::from("val"),
        }
    );

    assert_eq!(
        MaxLenOverwriteMsg::from_json_stack_errs_lang("{\"field\": \"val\"}".as_bytes(), "en")
            .unwrap(),
        MaxLenOverwriteMsg {
            field: String::from("val"),
        }
    );
}

#[test]
fn max_len_overwrite_msg_invalid() {
    assert_eq!(
        MaxLenOverwriteMsg::from_json(r#"{"field": "value"}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "too long",)
    );

    assert_eq!(
        MaxLenOverwriteMsg::from_json_lang(r#"{"field": "value"}"#.as_bytes(), "ar").unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "طويل جدا",)
    );

    assert_eq!(
        MaxLenOverwriteMsg::from_json_stack_errs(r#"{"field": "value"}"#.as_bytes()).unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "too long",)
    );

    assert_eq!(
        MaxLenOverwriteMsg::from_json_stack_errs_lang(r#"{"field": "value"}"#.as_bytes(), "en")
            .unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "too long",)
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
#[dflt_lang = "ar"]
struct MaxLenOverwriteMsgDfltLang {
    #[max_len(value = 4, msgs{en="too long", ar="طويل جدا"})]
    field: String,
}

#[test]
fn max_len_overwrite_msg_dflt_lang_valid() {
    assert_eq!(
        MaxLenOverwriteMsgDfltLang::from_json("{\"field\": \"val\"}".as_bytes()).unwrap(),
        MaxLenOverwriteMsgDfltLang {
            field: String::from("val"),
        }
    );

    assert_eq!(
        MaxLenOverwriteMsgDfltLang::from_json_lang("{\"field\": \"val\"}".as_bytes(), "en")
            .unwrap(),
        MaxLenOverwriteMsgDfltLang {
            field: String::from("val"),
        }
    );

    assert_eq!(
        MaxLenOverwriteMsgDfltLang::from_json_stack_errs("{\"field\": \"val\"}".as_bytes())
            .unwrap(),
        MaxLenOverwriteMsgDfltLang {
            field: String::from("val"),
        }
    );

    assert_eq!(
        MaxLenOverwriteMsgDfltLang::from_json_stack_errs_lang(
            "{\"field\": \"val\"}".as_bytes(),
            "en"
        )
        .unwrap(),
        MaxLenOverwriteMsgDfltLang {
            field: String::from("val"),
        }
    );
}

#[test]
fn max_len_overwrite_msg_dflt_lang_invalid() {
    assert_eq!(
        MaxLenOverwriteMsgDfltLang::from_json(r#"{"field": "value"}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "طويل جدا",)
    );

    assert_eq!(
        MaxLenOverwriteMsgDfltLang::from_json_lang(r#"{"field": "value"}"#.as_bytes(), "en")
            .unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "too long",)
    );

    assert_eq!(
        MaxLenOverwriteMsgDfltLang::from_json_stack_errs(r#"{"field": "value"}"#.as_bytes())
            .unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "طويل جدا",)
    );

    assert_eq!(
        MaxLenOverwriteMsgDfltLang::from_json_stack_errs_lang(
            r#"{"field": "value"}"#.as_bytes(),
            "en"
        )
        .unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "too long",)
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct MinLen {
    #[min_len(value = 6)]
    field: String,
}

#[test]
fn min_len_valid() {
    assert_eq!(
        MinLen::from_json("{\"field\": \"value1\"}".as_bytes()).unwrap(),
        MinLen {
            field: String::from("value1"),
        }
    );

    assert_eq!(
        MinLen::from_json_lang("{\"field\": \"value1\"}".as_bytes(), "en").unwrap(),
        MinLen {
            field: String::from("value1"),
        }
    );

    assert_eq!(
        MinLen::from_json_stack_errs("{\"field\": \"value1\"}".as_bytes()).unwrap(),
        MinLen {
            field: String::from("value1"),
        }
    );

    assert_eq!(
        MinLen::from_json_stack_errs_lang("{\"field\": \"value1\"}".as_bytes(), "en").unwrap(),
        MinLen {
            field: String::from("value1"),
        }
    );
}

#[test]
fn min_len_invalid() {
    assert_eq!(
        MinLen::from_json(r#"{"field": "value"}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(
            From::from("field"),
            Vec::new(),
            "the string value must be at least 6 bytes long",
        )
    );

    assert_eq!(
        MinLen::from_json_lang(r#"{"field": "value"}"#.as_bytes(), "en").unwrap_err(),
        from::Err::new_validation_err(
            From::from("field"),
            Vec::new(),
            "the string value must be at least 6 bytes long",
        )
    );

    assert_eq!(
        MinLen::from_json_stack_errs(r#"{"field": "value"}"#.as_bytes()).unwrap_err(),
        from::Errs::new_validation_err(
            From::from("field"),
            Vec::new(),
            "the string value must be at least 6 bytes long",
        )
    );

    assert_eq!(
        MinLen::from_json_stack_errs_lang(r#"{"field": "value"}"#.as_bytes(), "en").unwrap_err(),
        from::Errs::new_validation_err(
            From::from("field"),
            Vec::new(),
            "the string value must be at least 6 bytes long",
        )
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct MinLenOverwriteMsg {
    #[min_len(value = 6, msgs{en="too short", ar="قصير جدا"})]
    field: String,
}

#[test]
fn min_len_overwrite_msg_valid() {
    assert_eq!(
        MinLenOverwriteMsg::from_json("{\"field\": \"value1\"}".as_bytes()).unwrap(),
        MinLenOverwriteMsg {
            field: String::from("value1"),
        }
    );

    assert_eq!(
        MinLenOverwriteMsg::from_json_lang("{\"field\": \"value1\"}".as_bytes(), "en").unwrap(),
        MinLenOverwriteMsg {
            field: String::from("value1"),
        }
    );

    assert_eq!(
        MinLenOverwriteMsg::from_json_stack_errs("{\"field\": \"value1\"}".as_bytes()).unwrap(),
        MinLenOverwriteMsg {
            field: String::from("value1"),
        }
    );

    assert_eq!(
        MinLenOverwriteMsg::from_json_stack_errs_lang("{\"field\": \"value1\"}".as_bytes(), "en")
            .unwrap(),
        MinLenOverwriteMsg {
            field: String::from("value1"),
        }
    );
}

#[test]
fn min_len_overwrite_msg_invalid() {
    assert_eq!(
        MinLenOverwriteMsg::from_json(r#"{"field": "value"}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "too short",)
    );

    assert_eq!(
        MinLenOverwriteMsg::from_json_lang(r#"{"field": "value"}"#.as_bytes(), "ar").unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "قصير جدا",)
    );

    assert_eq!(
        MinLenOverwriteMsg::from_json_stack_errs(r#"{"field": "value"}"#.as_bytes()).unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "too short",)
    );

    assert_eq!(
        MinLenOverwriteMsg::from_json_stack_errs_lang(r#"{"field": "value"}"#.as_bytes(), "en")
            .unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "too short",)
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
#[dflt_lang = "ar"]
struct MinLenOverwriteMsgDfltLang {
    #[min_len(value = 6, msgs{en="too short", ar="قصير جدا"})]
    field: String,
}

#[test]
fn min_len_overwrite_msg_dflt_lang_valid() {
    assert_eq!(
        MinLenOverwriteMsgDfltLang::from_json("{\"field\": \"value1\"}".as_bytes()).unwrap(),
        MinLenOverwriteMsgDfltLang {
            field: String::from("value1"),
        }
    );

    assert_eq!(
        MinLenOverwriteMsgDfltLang::from_json_lang("{\"field\": \"value1\"}".as_bytes(), "en")
            .unwrap(),
        MinLenOverwriteMsgDfltLang {
            field: String::from("value1"),
        }
    );

    assert_eq!(
        MinLenOverwriteMsgDfltLang::from_json_stack_errs("{\"field\": \"value1\"}".as_bytes())
            .unwrap(),
        MinLenOverwriteMsgDfltLang {
            field: String::from("value1"),
        }
    );

    assert_eq!(
        MinLenOverwriteMsgDfltLang::from_json_stack_errs_lang(
            "{\"field\": \"value1\"}".as_bytes(),
            "en"
        )
        .unwrap(),
        MinLenOverwriteMsgDfltLang {
            field: String::from("value1"),
        }
    );
}

#[test]
fn min_len_overwrite_msg_dflt_lang_invalid() {
    assert_eq!(
        MinLenOverwriteMsgDfltLang::from_json(r#"{"field": "value"}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "قصير جدا",)
    );

    assert_eq!(
        MinLenOverwriteMsgDfltLang::from_json_lang(r#"{"field": "value"}"#.as_bytes(), "en")
            .unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "too short",)
    );

    assert_eq!(
        MinLenOverwriteMsgDfltLang::from_json_stack_errs(r#"{"field": "value"}"#.as_bytes())
            .unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "قصير جدا",)
    );

    assert_eq!(
        MinLenOverwriteMsgDfltLang::from_json_stack_errs_lang(
            r#"{"field": "value"}"#.as_bytes(),
            "ar"
        )
        .unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "قصير جدا",)
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct Len {
    #[len(value = 6)]
    field: String,
}

#[test]
fn len_valid() {
    assert_eq!(
        Len::from_json("{\"field\": \"value1\"}".as_bytes()).unwrap(),
        Len {
            field: String::from("value1"),
        }
    );

    assert_eq!(
        Len::from_json_lang("{\"field\": \"value1\"}".as_bytes(), "en").unwrap(),
        Len {
            field: String::from("value1"),
        }
    );

    assert_eq!(
        Len::from_json_stack_errs("{\"field\": \"value1\"}".as_bytes()).unwrap(),
        Len {
            field: String::from("value1"),
        }
    );

    assert_eq!(
        Len::from_json_stack_errs_lang("{\"field\": \"value1\"}".as_bytes(), "en").unwrap(),
        Len {
            field: String::from("value1"),
        }
    );
}

#[test]
fn len_invalid() {
    assert_eq!(
        Len::from_json(r#"{"field": "value"}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(
            From::from("field"),
            Vec::new(),
            "string must be 6 bytes long",
        )
    );

    assert_eq!(
        Len::from_json_lang(r#"{"field": "value"}"#.as_bytes(), "en").unwrap_err(),
        from::Err::new_validation_err(
            From::from("field"),
            Vec::new(),
            "string must be 6 bytes long",
        )
    );

    assert_eq!(
        Len::from_json_stack_errs(r#"{"field": "value"}"#.as_bytes()).unwrap_err(),
        from::Errs::new_validation_err(
            From::from("field"),
            Vec::new(),
            "string must be 6 bytes long",
        )
    );

    assert_eq!(
        Len::from_json_stack_errs_lang(r#"{"field": "value"}"#.as_bytes(), "en").unwrap_err(),
        from::Errs::new_validation_err(
            From::from("field"),
            Vec::new(),
            "string must be 6 bytes long",
        )
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct LenOverwriteMsg {
    #[len(value = 6, msgs{en="invalid length", ar="طول غير صالح"})]
    field: String,
}

#[test]
fn len_overwrite_msg_valid() {
    assert_eq!(
        LenOverwriteMsg::from_json("{\"field\": \"value1\"}".as_bytes()).unwrap(),
        LenOverwriteMsg {
            field: String::from("value1"),
        }
    );

    assert_eq!(
        LenOverwriteMsg::from_json_lang("{\"field\": \"value1\"}".as_bytes(), "en").unwrap(),
        LenOverwriteMsg {
            field: String::from("value1"),
        }
    );

    assert_eq!(
        LenOverwriteMsg::from_json_stack_errs("{\"field\": \"value1\"}".as_bytes()).unwrap(),
        LenOverwriteMsg {
            field: String::from("value1"),
        }
    );

    assert_eq!(
        LenOverwriteMsg::from_json_stack_errs_lang("{\"field\": \"value1\"}".as_bytes(), "en")
            .unwrap(),
        LenOverwriteMsg {
            field: String::from("value1"),
        }
    );
}

#[test]
fn len_overwrite_msg_invalid() {
    assert_eq!(
        LenOverwriteMsg::from_json(r#"{"field": "value"}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "invalid length",)
    );

    assert_eq!(
        LenOverwriteMsg::from_json_lang(r#"{"field": "value"}"#.as_bytes(), "ar").unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "طول غير صالح",)
    );

    assert_eq!(
        LenOverwriteMsg::from_json_stack_errs(r#"{"field": "value"}"#.as_bytes()).unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "invalid length",)
    );

    assert_eq!(
        LenOverwriteMsg::from_json_stack_errs_lang(r#"{"field": "value"}"#.as_bytes(), "ar")
            .unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "طول غير صالح",)
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
#[dflt_lang = "ar"]
struct LenOverwriteMsgDfltLang {
    #[len(value = 6, msgs{en="invalid length", ar="طول غير صالح"})]
    field: String,
}

#[test]
fn len_overwrite_msg_dflt_lang_valid() {
    assert_eq!(
        LenOverwriteMsgDfltLang::from_json("{\"field\": \"value1\"}".as_bytes()).unwrap(),
        LenOverwriteMsgDfltLang {
            field: String::from("value1"),
        }
    );

    assert_eq!(
        LenOverwriteMsgDfltLang::from_json_lang("{\"field\": \"value1\"}".as_bytes(), "en")
            .unwrap(),
        LenOverwriteMsgDfltLang {
            field: String::from("value1"),
        }
    );

    assert_eq!(
        LenOverwriteMsgDfltLang::from_json_stack_errs("{\"field\": \"value1\"}".as_bytes())
            .unwrap(),
        LenOverwriteMsgDfltLang {
            field: String::from("value1"),
        }
    );

    assert_eq!(
        LenOverwriteMsgDfltLang::from_json_stack_errs_lang(
            "{\"field\": \"value1\"}".as_bytes(),
            "en"
        )
        .unwrap(),
        LenOverwriteMsgDfltLang {
            field: String::from("value1"),
        }
    );
}

#[test]
fn len_overwrite_msg_dflt_lang_invalid() {
    assert_eq!(
        LenOverwriteMsgDfltLang::from_json(r#"{"field": "value"}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "طول غير صالح",)
    );

    assert_eq!(
        LenOverwriteMsgDfltLang::from_json_lang(r#"{"field": "value"}"#.as_bytes(), "en")
            .unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "invalid length",)
    );

    assert_eq!(
        LenOverwriteMsgDfltLang::from_json_stack_errs(r#"{"field": "value"}"#.as_bytes())
            .unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "طول غير صالح",)
    );

    assert_eq!(
        LenOverwriteMsgDfltLang::from_json_stack_errs_lang(
            r#"{"field": "value"}"#.as_bytes(),
            "en"
        )
        .unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "invalid length",)
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct Enum {
    #[r#enum(values = ["value1", "value2"])]
    field: String,
}

#[test]
fn enum_valid() {
    assert_eq!(
        Enum::from_json("{\"field\": \"value1\"}".as_bytes()).unwrap(),
        Enum {
            field: String::from("value1"),
        }
    );

    assert_eq!(
        Enum::from_json_lang("{\"field\": \"value2\"}".as_bytes(), "en").unwrap(),
        Enum {
            field: String::from("value2"),
        }
    );

    assert_eq!(
        Enum::from_json_stack_errs("{\"field\": \"value2\"}".as_bytes()).unwrap(),
        Enum {
            field: String::from("value2"),
        }
    );

    assert_eq!(
        Enum::from_json_stack_errs_lang("{\"field\": \"value1\"}".as_bytes(), "en").unwrap(),
        Enum {
            field: String::from("value1"),
        }
    );
}

#[test]
fn enum_invalid() {
    assert_eq!(
        Enum::from_json(r#"{"field": "value"}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(
            From::from("field"),
            Vec::new(),
            "value must be one of: [\"value1\", \"value2\"]",
        )
    );

    assert_eq!(
        Enum::from_json_lang(r#"{"field": "value"}"#.as_bytes(), "en").unwrap_err(),
        from::Err::new_validation_err(
            From::from("field"),
            Vec::new(),
            "value must be one of: [\"value1\", \"value2\"]",
        )
    );

    assert_eq!(
        Enum::from_json_stack_errs(r#"{"field": "value"}"#.as_bytes()).unwrap_err(),
        from::Errs::new_validation_err(
            From::from("field"),
            Vec::new(),
            "value must be one of: [\"value1\", \"value2\"]",
        )
    );

    assert_eq!(
        Enum::from_json_stack_errs_lang(r#"{"field": "value"}"#.as_bytes(), "en").unwrap_err(),
        from::Errs::new_validation_err(
            From::from("field"),
            Vec::new(),
            "value must be one of: [\"value1\", \"value2\"]",
        )
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct EnumOverwriteMsg {
    #[r#enum(values = ["value1", "value2"], msgs{en="invalid value", ar="قيمة غير صالحة"})]
    field: String,
}

#[test]
fn enum_overwrite_msg_valid() {
    assert_eq!(
        EnumOverwriteMsg::from_json("{\"field\": \"value2\"}".as_bytes()).unwrap(),
        EnumOverwriteMsg {
            field: String::from("value2"),
        }
    );

    assert_eq!(
        EnumOverwriteMsg::from_json_lang("{\"field\": \"value1\"}".as_bytes(), "en").unwrap(),
        EnumOverwriteMsg {
            field: String::from("value1"),
        }
    );

    assert_eq!(
        EnumOverwriteMsg::from_json_stack_errs("{\"field\": \"value2\"}".as_bytes()).unwrap(),
        EnumOverwriteMsg {
            field: String::from("value2"),
        }
    );

    assert_eq!(
        EnumOverwriteMsg::from_json_stack_errs_lang("{\"field\": \"value1\"}".as_bytes(), "en")
            .unwrap(),
        EnumOverwriteMsg {
            field: String::from("value1"),
        }
    );
}

#[test]
fn enum_overwrite_msg_invalid() {
    assert_eq!(
        EnumOverwriteMsg::from_json(r#"{"field": "value"}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "invalid value",)
    );

    assert_eq!(
        EnumOverwriteMsg::from_json_lang(r#"{"field": "value"}"#.as_bytes(), "ar").unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "قيمة غير صالحة",)
    );

    assert_eq!(
        EnumOverwriteMsg::from_json_stack_errs(r#"{"field": "value"}"#.as_bytes()).unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "invalid value",)
    );

    assert_eq!(
        EnumOverwriteMsg::from_json_stack_errs_lang(r#"{"field": "value"}"#.as_bytes(), "ar")
            .unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "قيمة غير صالحة",)
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
#[dflt_lang = "ar"]
struct EnumOverwriteMsgDfltLang {
    #[r#enum(values = ["value1", "value2"], msgs{en="invalid value", ar="قيمة غير صالحة"})]
    field: String,
}

#[test]
fn enum_overwrite_msg_dflt_lang_valid() {
    assert_eq!(
        EnumOverwriteMsgDfltLang::from_json("{\"field\": \"value1\"}".as_bytes()).unwrap(),
        EnumOverwriteMsgDfltLang {
            field: String::from("value1"),
        }
    );

    assert_eq!(
        EnumOverwriteMsgDfltLang::from_json_lang("{\"field\": \"value2\"}".as_bytes(), "en")
            .unwrap(),
        EnumOverwriteMsgDfltLang {
            field: String::from("value2"),
        }
    );

    assert_eq!(
        EnumOverwriteMsgDfltLang::from_json_stack_errs("{\"field\": \"value2\"}".as_bytes())
            .unwrap(),
        EnumOverwriteMsgDfltLang {
            field: String::from("value2"),
        }
    );

    assert_eq!(
        EnumOverwriteMsgDfltLang::from_json_stack_errs_lang(
            "{\"field\": \"value2\"}".as_bytes(),
            "en"
        )
        .unwrap(),
        EnumOverwriteMsgDfltLang {
            field: String::from("value2"),
        }
    );
}

#[test]
fn enum_overwrite_msg_dflt_lang_invalid() {
    assert_eq!(
        EnumOverwriteMsgDfltLang::from_json(r#"{"field": "value"}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "قيمة غير صالحة",)
    );

    assert_eq!(
        EnumOverwriteMsgDfltLang::from_json_lang(r#"{"field": "vawalue"}"#.as_bytes(), "en")
            .unwrap_err(),
        from::Err::new_validation_err(From::from("field"), Vec::new(), "invalid value",)
    );

    assert_eq!(
        EnumOverwriteMsgDfltLang::from_json_stack_errs(r#"{"field": "valuqwre"}"#.as_bytes())
            .unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "قيمة غير صالحة",)
    );

    assert_eq!(
        EnumOverwriteMsgDfltLang::from_json_stack_errs_lang(
            r#"{"field": "vqwrqalue"}"#.as_bytes(),
            "en"
        )
        .unwrap_err(),
        from::Errs::new_validation_err(From::from("field"), Vec::new(), "invalid value",)
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct Required {
    field: String,
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
    field: String,
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
    field: String,
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
struct TypeMismatch {
    field: String,
}

#[test]
fn type_mismatch() {
    assert_eq!(
        TypeMismatch::from_json(r#"{"field": 25}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: string, found: number",
        )
    );

    assert_eq!(
        TypeMismatch::from_json_lang(r#"{"field": 25}"#.as_bytes(), "en").unwrap_err(),
        from::Err::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: string, found: number",
        )
    );

    assert_eq!(
        TypeMismatch::from_json_stack_errs(r#"{"field": 25}"#.as_bytes()).unwrap_err(),
        from::Errs::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: string, found: number",
        )
    );

    assert_eq!(
        TypeMismatch::from_json_stack_errs_lang(r#"{"field": 25}"#.as_bytes(), "en").unwrap_err(),
        from::Errs::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: string, found: number",
        )
    );

    assert_eq!(
        TypeMismatch::from_json(r#"{"field": true}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: string, found: boolean",
        )
    );

    assert_eq!(
        TypeMismatch::from_json_lang(r#"{"field": false}"#.as_bytes(), "en").unwrap_err(),
        from::Err::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: string, found: boolean",
        )
    );

    assert_eq!(
        TypeMismatch::from_json_stack_errs(r#"{"field": true}"#.as_bytes()).unwrap_err(),
        from::Errs::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: string, found: boolean",
        )
    );

    assert_eq!(
        TypeMismatch::from_json_stack_errs_lang(r#"{"field": true}"#.as_bytes(), "en").unwrap_err(),
        from::Errs::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: string, found: boolean",
        )
    );

    assert_eq!(
        TypeMismatch::from_json(r#"{"field": {this will be ignored}}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: string, found: object",
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
            "expected: string, found: object",
        )
    );

    assert_eq!(
        TypeMismatch::from_json_stack_errs(r#"{"field": {because it does not care}}"#.as_bytes())
            .unwrap_err(),
        from::Errs::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: string, found: object",
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
            "expected: string, found: object",
        )
    );

    assert_eq!(
        TypeMismatch::from_json(r#"{"field": [this will be ignored]}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: string, found: array",
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
            "expected: string, found: array",
        )
    );

    assert_eq!(
        TypeMismatch::from_json_stack_errs(r#"{"field": [because it does not care]}"#.as_bytes())
            .unwrap_err(),
        from::Errs::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: string, found: array",
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
            "expected: string, found: array",
        )
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct TypeMismatchOverwriteMsg {
    #[type_mismatch_msgs{en="invalid type", ar="نوع غير صالح"}]
    field: String,
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
    field: String,
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
    field: String,
}

#[test]
fn not_null() {
    assert_eq!(
        NotNull::from_json(r#"{"field": null}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: string, found: null",
        )
    );

    assert_eq!(
        NotNull::from_json_lang(r#"{"field": null}"#.as_bytes(), "ar").unwrap_err(),
        from::Err::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: string, found: null",
        )
    );

    assert_eq!(
        NotNull::from_json_stack_errs(r#"{"field": null}"#.as_bytes()).unwrap_err(),
        from::Errs::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: string, found: null",
        )
    );

    assert_eq!(
        NotNull::from_json_stack_errs_lang(r#"{"field": null}"#.as_bytes(), "en").unwrap_err(),
        from::Errs::new_validation_err(
            From::from("field"),
            Vec::new(),
            "expected: string, found: null",
        )
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct NotNullOverwriteMsg {
    #[not_null_msgs{en="null is not allowed", ar="القيمة الخالية غير مسموحة"}]
    field: String,
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
    field: String,
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
    field: Null<String>,
}

#[test]
fn null() {
    assert_eq!(
        NullValue::from_json(r#"{"field": null}"#.as_bytes()).unwrap(),
        NullValue { field: Null::Null }
    );

    assert_eq!(
        NullValue::from_json(r#"{"field": "nice"}"#.as_bytes()).unwrap(),
        NullValue {
            field: Null::Some(String::from("nice")),
        }
    );

    assert_eq!(
        NullValue::from_json_lang(r#"{"field": null}"#.as_bytes(), "en").unwrap(),
        NullValue { field: Null::Null }
    );

    assert_eq!(
        NullValue::from_json_lang(r#"{"field": "nice"}"#.as_bytes(), "en").unwrap(),
        NullValue {
            field: Null::Some(String::from("nice")),
        }
    );

    assert_eq!(
        NullValue::from_json_stack_errs(r#"{"field": null}"#.as_bytes()).unwrap(),
        NullValue { field: Null::Null }
    );

    assert_eq!(
        NullValue::from_json_stack_errs(r#"{"field": "nice"}"#.as_bytes()).unwrap(),
        NullValue {
            field: Null::Some(String::from("nice")),
        }
    );

    assert_eq!(
        NullValue::from_json_stack_errs_lang(r#"{"field": null}"#.as_bytes(), "ar").unwrap(),
        NullValue { field: Null::Null }
    );

    assert_eq!(
        NullValue::from_json_stack_errs_lang(r#"{"field": "nice"}"#.as_bytes(), "ar").unwrap(),
        NullValue {
            field: Null::Some(String::from("nice")),
        }
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct OptionValue {
    field: Option<String>,
}

#[test]
fn option() {
    assert_eq!(
        OptionValue::from_json(r#"{"field7": null}"#.as_bytes()).unwrap(),
        OptionValue { field: None }
    );

    assert_eq!(
        OptionValue::from_json(r#"{"field": "nice"}"#.as_bytes()).unwrap(),
        OptionValue {
            field: Some(String::from("nice")),
        }
    );

    assert_eq!(
        OptionValue::from_json_lang(r#"{"fiel3d": null}"#.as_bytes(), "en").unwrap(),
        OptionValue { field: None }
    );

    assert_eq!(
        OptionValue::from_json_lang(r#"{"field": "nice"}"#.as_bytes(), "en").unwrap(),
        OptionValue {
            field: Some(String::from("nice")),
        }
    );

    assert_eq!(
        OptionValue::from_json_stack_errs(r#"{"fiel3d": null}"#.as_bytes()).unwrap(),
        OptionValue { field: None }
    );

    assert_eq!(
        OptionValue::from_json_stack_errs(r#"{"field": "nice"}"#.as_bytes()).unwrap(),
        OptionValue {
            field: Some(String::from("nice")),
        }
    );

    assert_eq!(
        OptionValue::from_json_stack_errs_lang(r#"{"fi7eld": null}"#.as_bytes(), "ar").unwrap(),
        OptionValue { field: None }
    );

    assert_eq!(
        OptionValue::from_json_stack_errs_lang(r#"{"field": "nice"}"#.as_bytes(), "ar").unwrap(),
        OptionValue {
            field: Some(String::from("nice")),
        }
    );
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct OptionNullValue {
    field: OptionNull<String>,
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
        OptionNullValue::from_json(r#"{"field": "nice"}"#.as_bytes()).unwrap(),
        OptionNullValue {
            field: OptionNull::Some(String::from("nice")),
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
        OptionNullValue::from_json_lang(r#"{"field": "nice"}"#.as_bytes(), "ar").unwrap(),
        OptionNullValue {
            field: OptionNull::Some(String::from("nice")),
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
        OptionNullValue::from_json_stack_errs(r#"{"field": "nice"}"#.as_bytes()).unwrap(),
        OptionNullValue {
            field: OptionNull::Some(String::from("nice")),
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
        OptionNullValue::from_json_stack_errs_lang(r#"{"field": "nice"}"#.as_bytes(), "en")
            .unwrap(),
        OptionNullValue {
            field: OptionNull::Some(String::from("nice")),
        }
    );
}

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

#[derive(Debug, PartialEq)]
#[from(json)]
struct MultiErr {
    field1: String,
    field2: String,
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

struct UserName;

impl ::from::Validator<String> for UserName {
    fn none(val: &String, path: &Path) -> Result<(), ValidationErr> {
        if val.len() > 5 {
            return Err(ValidationErr::new(
                From::from("username"),
                path.clone(),
                "invalid username",
            ));
        };
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
#[from(json)]
struct CustomValidator {
    #[validators(UserName)]
    username: String,
}

#[test]
fn custom_validator() {
    assert_eq!(
        CustomValidator::from_json(r#"{"username": "long name"}"#.as_bytes()).unwrap_err(),
        from::Err::new_validation_err(From::from("username"), Vec::new(), "invalid username",)
    );

    assert_eq!(
        CustomValidator::from_json_lang(r#"{"username": "long name"}"#.as_bytes(), "ar")
            .unwrap_err(),
        from::Err::new_validation_err(From::from("username"), Vec::new(), "invalid username",)
    );

    assert_eq!(
        CustomValidator::from_json_stack_errs(r#"{"username": "long name"}"#.as_bytes())
            .unwrap_err(),
        from::Errs::new_validation_err(From::from("username"), Vec::new(), "invalid username",)
    );

    assert_eq!(
        CustomValidator::from_json_stack_errs_lang(r#"{"username": "long name"}"#.as_bytes(), "en")
            .unwrap_err(),
        from::Errs::new_validation_err(From::from("username"), Vec::new(), "invalid username",)
    );
}
