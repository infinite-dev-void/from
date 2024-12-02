# from

A procedural macro that generates custom parsing and validation code for `structs`, driven by [attributes](https://doc.rust-lang.org/reference/attributes.html).

**Table of contents**

- [Thanks](#thanks)
- [features](#features)
- [Notes](#notes)
- [Breaking Changes](#breaking-changes)
- [Minimum Supported Rust Version (MSRV)](#minimum-supported-rust-version-msrv)
- [Getting Started](#getting-started)
- [Performance](#performance)
- [How It Works](#how-it-works)
  - [FromJson](#fromjson)
  - [Return Type](#return-type)
  - [Example](#example)
  - [Supported field types and enums](#supported-field-types-and-enums)
    - [Types](#types)
    - [Enums](#enums)
- [Customization](#customization)
  - [String-specific attributes](#string-specific-attributes)
  - [String-Vec-specific attributes](#string-vec-specific-attributes)
  - [Numeric-specific attributes](#numeric-specific-attributes)
  - [Float-specific attributes](#float-specific-attributes)
  - [Boolean-specific attributes](#boolean-specific-attributes)
  - [Vec-specific attributes](#vec-specific-attributes)
  - [Struct-specific attributes](#struct-specific-attributes)
  - [Unspecific attributes](#unspecific-attributes)
- [How to define custom validator](#how-to-define-custom-validator)
- [Error Message Customization](#error-message-customization)
- [Special behavior](#special-behavior)
- [Examples](#examples)
- [FAQ/Troubleshooting](#faqtroubleshooting)
- [Contribution](#contribution)
- [License Information](#license-information)

## Thanks

I would like to thank `ChatGPT` for helping me write this document as well as helping me design some parts of this crate.

## Features

1- Simple and intuitive design without sacrificing performance.
2- Utilizes the power of Rust procedural macros to reduce boilerplate.
3- Allows constraints to be easily added to fields for validation.
4- Generates clear `compile_error` messages for incorrectly typed attributes, catching issues early.
5- Combines parsing and validation into a single, efficient step.
6- Supports customizable validation error messages in multiple languages.
7- Returns detailed error messages from the parser function if there is a validation or syntax error.

## Notes

- This crate currently supports only `JSON` format.
- `from` macro can only be used on `structs` with named fields..
- There is a plan to support more data formats and other data structures in the future.

## Breaking Changes

Future minor updates may introduce changes that are not backward compatible until the first major version release.

## Minimum Supported Rust Version (MSRV)

This crate supports Rust version 1.61.0 and later. While it may work with older versions, this is not guaranteed.

## Getting Started

First, in your Cargo.toml file add the following:

```toml
[dependencies]
from = "~0.1.0"
```

Next, add the following to your src/main.rs file:

```rust
use from::{from, FromJson};

#[derive(Debug, PartialEq)] // Unrelated attributes must precede 'from'
#[from(json)] // Define parsing format (JSON in this case)
struct Developer {
    name: String,
    age: u8,
    languages: Vec<String>,
    freelancer: bool,
    github: Option<String>,
}

fn main(){
    let json = r#"{
        "name": "Ali",
        "age": 25,
        "languages": ["Rust", "C++"],
        "freelancer": false
    }"#.as_bytes();

    assert_eq!(
        Developer::from_json(json),
        Ok(Developer {
            name: String::from("Ali"),
            age: 25,
            languages: vec!["Rust", "C++"],
            freelancer: false,
            github: None,
        })
    );
}
```

For more detailed examples see [Examples](https://github.com/infinite-dev-void/from?tab=readme-ov-file#examples) section.

## Performance

The `from` macro is designed with performance in mind. It generates highly optimized parsing and validation code at compile time, ensuring minimal runtime overhead. it also reduces boilerplate and redundant checks.

**Parsing Performance**

The parsing performance of `from(json)` is close to or on par with `serde_json`. While `serde_json` outperforms `from(json)` in certain cases, such as parsing floats, `from(json)` shows better performance in other cases, like parsing integers. As a result, the overall performance of `from(json)` is comparable to that of `serde_json` for typical use cases.

For detailed benchmark results, see [benches](https://github.com/infinite-dev-void/from/tree/main/benches) folder (Benchmarked using `criterion` crate).

## How It Works

To use `from` macro, add it directly above the struct definition, following any unrelated attributes (as shown in the example above). Second we have to specify which data format we want to parse the struct from (accept multiple values. However, currently only `JSON` format is supported).

When `#[from(json)]` is added, the macro analyzes the struct and implements the `FromJson` and `FromJsonValue` traits for it. The key trait is `FromJsonValue`, as `FromJson` has a default implementation. The methods of the `FromJsonValue` trait are generated by the macro, allowing it to customize the parsing logic based on the struct's fields and any attributes you provide.

see [Supported field types and enums](https://github.com/infinite-dev-void/from?tab=readme-ov-file#supported-field-types-and-enums) section for information about compatibility.

### FromJson

`FromJson` trait provides 4 methods as follows:

```rust
pub trait FromJson: FromJsonValue {
    fn from_json(json: &[u8]) -> Result<Self, Err> {
        /* Omitted */
    }

    fn from_json_lang(json: &[u8], lang: &str) -> Result<Self, Err> {
        /* Omitted */
    }

    fn from_json_stack_errs(json: &[u8]) -> Result<Self, Errs> {
        /* Omitted */
    }
    fn from_json_stack_errs_lang(json: &[u8], lang: &str) -> Result<Self, Errs> {
        /* Omitted */
    }
}
```

**from_json**

This method takes a json input and returns a `Result` containing either the parsed struct (on success) or an `Err` enum (on failure). Any error that occurs during the parsing (validation or syntax error) will be returned immediately.

Note: If you overwrite an error message the one will be picked is the one of "en" language or the one of the language you specified using `dflt_lang` attribute (see customization section).

**from_json_lang**

This method takes two argument the first one is `json` the second is `lang` which determine the language of the error messages in case if you overwrite one of them.

**from_json_stack_errs**

This is the same as `from_json` except that it will return `Errs` enum (if failed). in other words if a validation error occurs during the parsing it will be pushed to a vector and the parsing process will be continue until the end of the `json` input and then all the errors will be returned (if there). any syntax error will be returned immediately.

**from_json_stack_errs_lang**

This is the same as `from_json_stack_errs` but takes `lang` as a second argument which determine the language of the error messages in case if you overwrite one of them.

### Return Type

All parsing methods return either `Result<Self, Err>` or `Result<Self, Errs>` where `Self` is the struct while `Err` and `Errs` are enums and defined as follows:

**Err enum**

```rust
pub enum Err {
    SyntaxErr(SyntaxErr),
    ValidationErr(ValidationErr),
}
```

**Errs enum**

```rust
pub enum Errs {
    SyntaxErr(SyntaxErr),
    ValidationErrs(Vec<ValidationErr>),
}
```

Both of `Err` and `Errs` have `SyntaxErr` and `ValidationErr` as a variant which are defined as follows:

**SyntaxErr type**

```rust
pub struct SyntaxErr {
    pub msg: String,
    pub offset: usize, // Number of bytes from the beginning of the input to the error location
}
```

**ValidationErr type**

```rust
pub struct ValidationErr {
    pub target: PropOrIdx, // which property contains the error
    pub path: Path, // the path to this property if you have nested objects and arrays
    pub msg: String,
}
```

**PropOrIdx enum**

```rust
pub enum PropOrIdx {
    Prop(String),
    Idx(usize),
}
```

**Path**

```rust
pub type Path = Vec<PropOrIdx>;
```

### Example

The following example demonstrates the use of `Err` and `Errs`.

```rust
use from::{from, FromJson};

#[from(json)]
struct Example {
    id: u32,
    hobbis: Vec<String>,
}


fn main(){
    // if `from_json` find any validation error it will return it immediately.
    assert_eq!(
        Example::from_json(
            r#"{
                "id": "my name",
                "hobbies": 15
            }"#.as_bytes(),
        ),
        Err(from::Err::ValidationErr(
            from::ValidationErr::new(
                from::PropOrIdx::Prop("id"), // target
                vec![], // path
                "expected: u32, found: string", // msg
            ),
        )),
    );

    assert_eq!(
        Example::from_json_stack_errs(
            r#"{
                "id": -15,
                "hobbis": ["Programming", 15]
            }"#.as_bytes(),
        ),
        Err(from::Errs::ValidationErrs(
            vec![
                from::ValidationErr::new(
                    from::PropOrIdx::Prop("id"), // can be replaced by From::from("id")
                    vec![],
                    "number is too small to fit in 'u32' type",
                ),
                from::ValidationErr::new(
                    from::PropOrIdx::Idx(1),
                    vec![from::PropOrIdx::Prop("hobbies")],
                    "expected: string, found: number",
                ),
            ],
        )),
    );

    assert_eq!(
        Example::from_json(
            // missing colon after (25) ... must be (25,)
            r#"{"id": 25 "hobbies": []}"#.as_bytes(),
        ),
        Err(from::Err::SyntaxErr(
            from::SyntaxErr {
                msg: String::from("expected: ',' or '}', found: \""),
                offset: 10,
            },
        ))
    );
}
```

### Supported field types and enums

#### Types

- String.
- integers (i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize).
- floats (f32, f64).
- Vector containing:
  - anything in this list.
  - Null enum.
- custom (anything implements `FromJsonValue` trait).

#### Enums

- Option.
- Null.
- OptionNull.

`Null` and `OptionNull` are provided by the crate to handle scenarios involving null or missing values. The `Null` enum is used to represent cases where a value can explicitly be null. The built-in `Option` enum is utilized to signify the absence of a value. Meanwhile, `OptionNull` combines the capabilities of both, allowing you to manage scenarios where a value can be either null or entirely missing. see the following example:

**Example**

```rust
#[from(json)]
struct Example {
    field1: Option<String>, // this make the field optional

    #[max(value = 25)] // you can use them with `Null`, `Option` and `OptionNull`
    field2: Null<u16>, // this make the field accept either null or u16 value,


    field3: OptionNull<f64>, // let the field accept either: the absence of value, null or f64.
}
```

## Customization

To tell the macro to customize parsing logic, `attributes` are used. Below are definitions and explanations of the supported attributes.

### String-specific attributes

The following attributes is used only with `String` type.

1- **trim_start**
Used to trim whitespaces at the beginning of the string (use `.trim_start()` internally)

```rust
#[from(json)]
struct Example {
    #[trim_start]
    field: String,
}
```

2- **trim_end**
Used to trim whitespaces at the end of the string (use `.trim_end()` internally)

```rust
#[from(json)]
struct Example {
    #[trim_end]
    field: String,
}
```

3- **trim**
Used to trim whitespaces at both the beginning and end of the string (use `.trim()` internally).

```rust
#[from(json)]
struct Example {
    #[trim_start] // <- this will be ignored
    #[trim_end] // <- this will be ignored
    #[trim]
    field: String,
}
```

4- **sanitize_xss**
Used to sanitize string from `xss`.

NOTE: This will increase the length of the `String` because for example this `<` will be converted to `&lt;`.

```rust
#[from(json)]
struct Example {
    #[sanitize_xss]
    field: String,
}
```

### String-Vec-specific attributes

The following attributes can be used with both `String` and `Vec`.

1- **max_len**
Used to specify the maximum allowed length (use `.len()` method internally) and it has two sub-attributes: value (to set the maximum length) and msgs (optional, to customize error messages). This attribute can be defined as follows:

```rust
#[from(json)]
struct Example {
    #[max_len(
        value=5,
        msgs{ // optional
            en = "too long",
            ar = "طويل جدا",
        },
    )]
    field: String,
}
```

2- **min_len**
Used to specify the minimum allowed length (use `.len()` method internally). an it has two attributes exactly as `max_len`. this attribute can be defined as follows:

```rust
#[from(json)]
struct Example {
    #[min_len(
        value=7,
        msgs{ // optional
            en = "too long",
            ar = "طويل جدا",
        },
    )]
    field: Vec<u8>,
}
```

3- **len**
Used to specify a fixed allowed length (use `.len()` method internally). `from` will ignore `max_len` and `min_len` attributes if this attribute has been added.

```rust
#[from(json)]
struct Example {
    #[max_len(/* Omitted */)] // <- this will be ignored
    #[min_len(/* Omitted */)] // <- this will be ignored
    #[len(
        value=7,
        msgs{ // optional
            en = "too long",
            ar = "طويل جدا",
        },
    )]
    field: Vec<u32>,
}
```

### Numeric-specific attributes

The following attributes can be used with integers or floats or the both of two.

1- **max**
Used with integer and float types to specify the maximum allowed value. `inclusive` is an additional optional sub-attribute that can be added with **float** types to use `val >= max` instead of `val > max` for comparison..

```rust
#[from(json)]
struct Example {
    #[max(
        value = 10.0,
        msgs{ // optional
            en = "too large",
            ar = "كبير جدا",
        },
        inclusive, // optional, only with floats
    )]
    field: f32,
}
```

2- **min**
Used with integer and float types to specify the minimum allowed value. `inclusive` is an additional optional sub-attribute that can be added with **float** types to use `val <= max` instead of `val < max` for comparison..

```rust
#[from(json)]
struct Example {
    #[min(
        value = 7.0,
        msgs{ // optional
            en = "too small",
            ar = "صغير جدا",
        },
        inclusive, // optional, only with floats
    )]
    field: u128,
}
```

### Float-specific attributes

1- **max_fracs**
Used to specify the maximum allowed fraction digits (number of digits after the decimal point).

```rust
#[from(json)]
struct Example {
    #[max_fracs(
        value = 3,
        msgs{ // optional
            en = "too many fraction digits",
            ar = "عدد كبير جدا من الأرقام الكسرية",
        }
    )]
    field: f64,
}
```

NOTE: the process of determine the number of fraction digits is done after parsing the float number by converting the parsed number `.to_string()` to ensure the accuracy.

2- **allow_infinite**
Used only with float types to indicate that infinite float number (`Infinite`, `-Infinite`) are allowed. The word `Inf` or `Infinite` is not a valid `JSON` value so it is not possible to get `Infinite` value using this, but by a very large number like `7e887` or a very large negative number like `-8e989` where these numbers are considered `Infinite` by `f32` and `f64`.

Note: `f64` has more capacity than `f32` to hold larger numbers before the number is considered as `Infinite` (see Rust [f64](https://doc.rust-lang.org/std/primitive.f64.html) and [f32](https://doc.rust-lang.org/std/primitive.f32.html) types for more information).

```rust
#[from(json)]
struct Example {
    #[allow_infinite]
    field: f64,
}
```

3- **infinite_msgs**
Used only with float types to overwrite the default error message of an `Infinite` number. this attribute will be ignored if `allow_infinte` is present.

```rust
#[from(json)]
struct Example {
    #[infinite_msgs{
        en = "infinite number is not allowed",
        ar = "غير مسموح بالأعداد اللانهائية",
    }]
    field: f32,
}
```

### Boolean-specific attributes

1- **must_be**
Used to enforce the value to be either: `true` or `false`.

```rust
#[from(json)]
struct Example {
    #[must_be(
        value = true,
        msgs{ // optional
            en = "value must be true",
            ar = "القيمة يجب أن تكون 'true'",
        },

    )]
    field: bool,
}
```

### Vec-specific attributes

1- **elem**
Used to customize the parsing of the element contained within the vector. Suppose that there is a vector of `usize` and it must be be within a specifc range then `elem` attribute can be used to do this and the sub-attributes inside it should be compatible with that type (i.e: for integer types use (`max`, `min`, ...etc), for `String` type use (`max_len`, `min_len`,...etc), for `Vec` type use (`max_len`, `elem`, ...etc)).

```rust
#[from(json)]
struct Example {
    #[elem(
        max(
            value = 25,
        ),
        min(
            value = 5,
        ),
    )]
    field: Vec<usize>,
}
```

Note: If there is a nested vector `Vec<Vec<String>>`, the `elem` attribute can be used nestedly as well like this:

```rust
#[from(json)]
struct Example {
    #[elem(
        elem(
            max_len(value = 5)
        )
    )] // there is no limit to the depth of the `elem` attribute
    field: Vec<Vec<String>>,
}
```

### Struct-specific attributes

1- **dflt_lang**
Used with the struct itself to overwrite the default language ("en") used to pick the appropriate error message

```rust
#[from(json)]
#[dflt_lang="ar"]
struct Example {
    #[required_msgs{
        en = "required field",
        ar = "حقل مطلوب",
    }]
    field: u16,
}
```

### Unspecific attributes

1- **r#enum**
Used with `String` and any integer or float type to specify a list of allowed values.

```rust
#[from(json)]
struct Example {
    #[r#enum(
        values = [10, 7, 18],
        msgs{ // optional
            en = "only 10, 7 and 18 are allowed",
            ar = "مسمحوح فقط بالقيمة 10, 7 أو 18",
        },
    )]
    field: u32,
}
```

2- **validators**
Used with `custom`, `String`, integers, floats, `bool` and `Vec` types. (see **How to define custom validator** section for more details).

NOTE: `validators` accept any number of validators (`validators(vald1, vald2, vald3,...etc)`).

```rust

#[from(json)]
struct User {/* Omitted */}

struct UserNameValidator;

impl ::from::Validator<String> for UserNameValidator { /* more details later */}

#[from(json)]
struct Example {
    #[validators(UserNameValidator)]  // if the validator in another module or crate it is preferable to write the abosulte path to it
    username: String,
}
```

3- **default**
Used with `String`, `bool`, integers and floats to specify a default value in case the value is missing in the input data. This attribute will be ignored if the type is wrapped by `Option` or `OptionNull` enums. (It is not logical to define a default value while the field itself is optional)

```rust
#[from(json)]
struct Example {
    #[default = 20]
    field1: u16

    #[default = "nice to meet you"] // <- this will be ignored
    field2: Option<String>,
}
```

4- **required_msgs**
Used with all supported types to overwrite `required` error message. This attribute will be ignored if the type is wrapped by `Option` or `OptionNull` enums.

```rust
#[from(json)]
struct Example {
    #[required_msgs{
        en = "required field",
        ar = "حقل مطلوب",
    }]
    field1: u32,

    #[required_msgs{/* Omitted */}] // <- this will be ignored
    field2: ::from::OptionNull<String>,
}
```

5- **not_null_msgs**
Used with all supported types to overwrite `not_null` error message. This attribute will be ignored if the type wrapped with `Null` or `OptionNull` enums.

```rust
#[from(json)]
struct Example {
    #[not_null_msgs {
        en = "null is not allowed",
        ar = "القيمة الفارغة ليست مسموحة",
    }]
    field: u16,

    #[not_null_msgs{/* Omitted */}] // <- this will be ignored
    nullable: ::from::Null<u16>,
}
```

6- **type_mismatch_msgs**
Used with all supported types to overwrite `type_mismatch` error message.

```rust
#[from(json)]
struct Example {
    #[type_mismatch_msgs{
        en = "invalid type",
        ar = "نوع غير صالح",
    }]
    field: u16,
}
```

## How to define custom validator

To create a custom validator named "TestValidator" for example. first, create a unit struct named "TestValidator" then implement `Validator` trait for it:

**Validator**

```rust
pub trait Validator<V> {
    fn none(val: &V, path: &Path) -> Result<(), ValidationErr>;

    #[inline(always)]
    fn lang(val: &V, path: &Path, _lang: &str) -> Result<(), ValidationErr> {
        // default implementation...
    }

    #[inline(always)]
    fn stack_errs(val: &V, path: &Path) -> Result<(), Vec<ValidationErr>> {
        // default implementation...
    }

    #[inline(always)]
    fn stack_errs_lang(val: &V, path: &Path, _lang: &str) -> Result<(), Vec<ValidationErr>> {
        // default implementation...
    }
}
```

This trait have a generic argument used to specify the type of the value that need to be validated. also it contains four methods (`none`, `lang`, `stack_errs`, `stack_errs_lang`) each of them is called by the corresponding parser (i.e. `none` is called by `from_json`, `lang` is called by `from_json_lang`, .. etc.) . only `none` method need to be defined while the other methods will take the default implementation (you can overwrite them if you want). However, after implementing the trait the name of the unit struct `TestValidator` must be use as a sub-attribute in `validators` attribute as follows.

```rust
struct TestValidator;

impl ::from::Validator<String> for TestValidator {
    fn none(
        val: &String, // <- type you want to validate
        path: &::from::Path, // path to this value if nested otherwise empty vector.
    ) -> Result<(), ::from::ValidationErr> {
        /* your logic */
    }

    // can be overwritten
    fn stack_errs_lang(
        val: &String,
        path: &::from::Path,
        lang: &str,
    ) -> Result<(), Vec<::from::ValidationErr>> {
        /* your logic */
    }
}

#[from(json)]
struct Example {
    #[validators(TestValidator)]
    field: String,

    // can be used with `Option`, `Null` and `OptionNull` and it will be called
    // only in case of `Some`
    // #[validators(TestValidator)]
    // field: Option<String>,
}
```

## Error Message Customization

Error message can be customized in multiple ways (as mentioned in [Customization](https://github.com/infinite-dev-void/from?tab=readme-ov-file#customization)). However, there are some important things must be considered:

1- All default messages are in English.
2- These messages are assigned to `en` key **or** to the value specified in `dflt_lang` attribute. (i.e: if `dflt_lang = "fr"` then all default messages will be assigned to `fr` key).
3- To overwrite a default message in any validation case (as mentioned in [Customization](https://github.com/infinite-dev-void/from?tab=readme-ov-file#customization)) just use the same key as `dflt_lang` (if not specified use `en`).
4- if the user entered a language key that not defined then the default error message (or overwritten one) will be matched.
5- there are no limitaions to the language key.
6- `from_json` and `from_json_stack_errs` methods take the default message (or overwritten one) since `lang` parameter is not there.

NOTE: some of the default messages are dynamically generated based on the field type and input data. For example:

- For a `String` and type mismatch input (number): "expected: string, found number".
- For a `usize` and null input: "expected: usize, found: null".

## Special behavior

Extra fields in the JSON input are skipped without validation, ensuring unused fields do not impact performance. This behavior optimizes parsing by avoiding unnecessary processing.

```rust
#[from(json)]
struct Example {
    name: String,
}

fn main(){
    assert_eq!(
        Example::from_json(r#"{
            "field": "ok",
            "field2": [this is not a valid [array] "this -> ] <- is not the end"],
            "feild3": {this too is not a valid {object} "this -> } is not the end"}
        }"#),
        Ok(Example {
            name: String::from("ok"),
        }),
    );
}
```

## Examples

In the following two examples, the use of `Option`, `Null`, `OptionNull` enums will be demonstrated, as will the use of the available attributes that customize the parsing process.

First, suppose that `AddPerson` struct is the one used if you want to add "person" to the "Database" for example. rather than parsing the data and then validating it manually (like: this field is required, this one is optional, this must not be very long...etc), `from` macro can be used to make it a very easy process (see example 1).

**Example 1**

```rust
use from::{from, FromJson, Null, ValidationErr};

#[derive(Debug, PartialEq)]
#[from(json)]
struct AddPerson {
    #[max_len(
        value = 20,
        msgs{
            en = "name must contain at least 20 characters",
            ar = "يجب أن يحتوي الاسم على 20 حرفًا على الأقل",
        }
    )]
    #[trim]
    name: String,

    #[max(value = 120)]
    #[default = 24]
    age: u8,

    #[max(value = 290)]
    #[max_fracs(value = 1)]
    length: f32,

    job: Option<String>,

    #[min(value = 1)]
    #[default = null]
    experience: Null<u8>, // Use `Null` if you want to force the field to exist.

    #[elem(
        #[r#enum(values = ["Arabic", "English"])]
    )]
    languages: Option<Vec<String>>,
}

fn main(){
    assert_eq!(
        AddUser::from_json_stack_errs_lang(
            r#"{
                "name": "Velit veniam ullamco mollit",
                "age": 140,
                "length": 180.75,
                "experience": 0,
                "languages": ["Arabic", "German"]
            }"#.as_bytes(),
            "en",
        ),
        Err(from::Errs::ValidationErrs(vec![
            ValidationErr::new(
                From::from("name"), // target
                vec![], // path
                "name must contain at least 20 characters", // msg
            ),
            ValidationErr::new(
                From::from("age"),
                vec![],
                "number must be less than or equal to 120",
            ),
            ValidationErr::new(
                From::from("length"),
                vec![],
                "fraction digits must not be more than 1",
            ),
            ValidationErr::new(
                From::from("experience"),
                vec![],
                "number must be greater than or equal to 1",
            ),
            ValidationErr::new(
                From::from(1),
                vec![From::from("languages")],
                r#"value must be one of: ["Arabic", "English"]"#,
            ),
        ])),
    );

    assert_eq!(
        AddUser::from_json_stack_errs(
            r#"{
                "name": "Mohammed Ali",
                "age": 75,
                "length": 191,
                "job": "Boxer"
                "experience": 21,
                "languages": ["English", "Arabic"]
            }"#.as_bytes(),
        ),
        Ok(AddUser {
            name: String::from("Mohammed Ali"),
            age: 75,
            length: 191.0,
            job: Some(String::from("Boxer")),
            experience: Null::Some(21),
            languages: Some(vec![String::from("English"), String::from("Arabic")])
        }),
    );
}

```

**Example 2**

Example 1 demonstrated the most cases. However, the case where `OptionNull` enum is used not demonstrated. so, in example 2 this will be covered. suppose that the `UpdatePerson` struct is the one used to update the person you added. but you only want to update only the field the user will provide (i.e: if the user want to update only the "name" field, then the json input will contains only that field"). To do this `UpdatePerson` struct will be as follows:

```rust
use from::{from, FromJson, OptionNull, ValidationErr};

#[derive(Debug, PartialEq)]
#[from(json)]
struct UpdatePerson {
    #[max_len(
        value = 20,
        msgs{
            en = "name must contain at least 20 characters",
            ar = "يجب أن يحتوي الاسم على 20 حرفًا على الأقل",
        }
    )]
    #[trim]
    name: Option<String>,

    #[max(value = 120)]
    #[default = 24]
    age: Option<u8>,

    #[max(value = 290)]
    #[max_fracs(value = 1)]
    length: f32,

    job: OptionNull<String>,

    #[min(value = 1)]
    #[default = null]
    experience: OptionNull<u8>,

    #[elem(
        #[r#enum(values = ["Arabic", "English"])]
    )]
    languages: OptionNull<Vec<String>>,
}
```

Notice that all `Option` fields is converted to `OptionNull` to handle the case where you want to set the value to a new one (`Some`), set it to null (`Null`) or ignore it (`None`).

```rust
fn main(){
    assert_eq!(
        UpdatePerson::from_json_lang(
            r#"{
                "age": 70,
                "length": 194,
                "experience": null,
                "languages": null
            }"#.as_bytes(),
            "en"
        ),
        Ok(UpdatePerson {
            name: None,
            age: Some(70),
            length: Some(197),
            job: None,
            experience: OptionNull::Null,
            languages: OptionNull::Null,
        }),
    );
}
```

## FAQ/Troubleshooting

Can I use this crate with formats other than JSON?

- Currently, only `json` format is supported. But in the near future other formats will be added.

What happens if a field is missing in the input data?

- Depends. If the field is defined using `Option` or `OptionNull` enums nothing will happen and the field value will be `None`, otherwise a validation error will be returned (or pushed) indicating that the field is required.

What happens if a field is duplicated in the input data?

- Currently, the parsing and validation process will be repeated, causing a validation error or value substitution. However, this may change in the future.

## Contribution

If you encounter any issues or want to suggest a feature, please open an [issue](https://github.com/infinite-dev-void/from/issues) in github.

## License Information

"from" is licensed under "from" Ethical License. see [LICENSE](https://github.com/infinite-dev-void/from/tree/main/LICENSE) for full license details.
