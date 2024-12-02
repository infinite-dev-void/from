use metas_holder::MetasHolder;
use proc_macro::TokenStream;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use syn::{
    parse, parse_macro_input, punctuated::Punctuated, Data, DeriveInput, Fields, FieldsNamed,
    Generics, Ident, Path, Token,
};

//
//
//

mod utils;
use utils::Append;

//
//
//

mod custom_types;
use custom_types::*;

//
//
//

mod metas_holder;

//
//
//

mod json;
use json::FromJsonValueImpl;

//
//
//

mod kind;
use kind::{Kind, Type};

//
//
//

mod types;

#[proc_macro_attribute]
pub fn from(attribute: TokenStream, input: TokenStream) -> TokenStream {
    let opts = match get_from_opts(attribute) {
        Ok(opts) => opts,
        Err(ts) => return ts,
    };

    match _from(parse_macro_input!(input as DeriveInput), opts) {
        Ok(ts) => ts,
        Err(ts) => ts,
    }
}

//
//
//
//
//
//
//

fn get_from_opts(attribute: TokenStream) -> Result<u8, TokenStream> {
    let paths =
        match parse::Parser::parse(Punctuated::<Path, Token![,]>::parse_terminated, attribute) {
            Ok(attrs) => attrs,
            Err(e) => return Err(TokenStream::from(e.to_compile_error())),
        };

    let mut from = 0b0000_0000u8;

    for path in &paths {
        if path.is_ident("json") {
            from |= 0b0000_0001;
            continue;
        };

        /* if path.is_ident("protobuf") {
            from |= 0b0000_0010;
        }; */
    }

    match from {
        0 => Err(utils::compile_err(
            &paths,
            "at least one of the following must be selected:\n - json\n - protobuf",
        )),

        _ => Ok(from),
    }
}

//
//
//
//
//
//
//

fn _from(input: DeriveInput, opts: u8) -> Result<TokenStream, TokenStream> {
    let data_struct = match input.data {
        Data::Struct(s) => s,

        _ => {
            return Err(utils::compile_err(
                &input,
                "only named structs are supported",
            ))
        }
    };

    //
    //
    //
    //
    //

    let fields = match data_struct.fields {
        Fields::Named(fields) => fields,

        _ => {
            return Err(utils::compile_err(
                &data_struct.fields,
                "only named structs are supported",
            ))
        }
    };

    let dflt_lang = match input.attrs.parse_value_if_found::<String>("dflt_lang")? {
        Some((dflt_lang, _)) => dflt_lang,
        None => String::from("en"),
    };

    let struct_ident = input.ident;
    let generics = input.generics;

    let struct_fields;
    let impls;

    match opts {
        0b01 => {
            (struct_fields, impls) =
                build_struct_and_from_json_impl(&struct_ident, &generics, fields, &dflt_lang)?;
        }

        _ => {
            debug_assert!(opts < 0b10);
            unsafe {
                std::hint::unreachable_unchecked();
            };
        }
    }
    //
    //
    //
    //
    //
    //
    //

    let vis = input.vis;

    let mut struct_def = quote! {#vis struct #struct_ident #generics{#struct_fields}};

    struct_def.append(impls);

    Ok(struct_def.into())
}

fn build_struct_and_from_json_impl(
    struct_ident: &Ident,
    generics: &Generics,
    fields: FieldsNamed,
    dflt_lang: &str,
) -> Result<(TokenStream2, TokenStream2), TokenStream> {
    let mut struct_fields = TokenStream2::new();
    let mut json_impl = FromJsonValueImpl::new();

    let mut kind;
    let mut field_ident;
    let mut fields = fields.named.into_iter();
    let mut field;

    loop {
        field = match fields.next() {
            Some(field) => field,
            None => break,
        };

        kind = Kind::from_ty(&field.ty)?;
        let attrs = field.attrs;
        field.attrs = Vec::new();

        field_ident = FieldIdent::new(&field);

        struct_fields.append(quote! {#field,});

        match kind.ty {
            Type::I8 => json_impl.add_int_field::<i8>(
                field_ident,
                attrs,
                dflt_lang,
                kind.option,
                kind.null,
            )?,

            Type::I16 => json_impl.add_int_field::<i16>(
                field_ident,
                attrs,
                dflt_lang,
                kind.option,
                kind.null,
            )?,

            Type::I32 => json_impl.add_int_field::<i32>(
                field_ident,
                attrs,
                dflt_lang,
                kind.option,
                kind.null,
            )?,

            Type::I64 => json_impl.add_int_field::<i64>(
                field_ident,
                attrs,
                dflt_lang,
                kind.option,
                kind.null,
            )?,

            Type::I128 => json_impl.add_int_field::<i128>(
                field_ident,
                attrs,
                dflt_lang,
                kind.option,
                kind.null,
            )?,

            Type::ISize => json_impl.add_int_field::<isize>(
                field_ident,
                attrs,
                dflt_lang,
                kind.option,
                kind.null,
            )?,

            Type::U8 => json_impl.add_int_field::<u8>(
                field_ident,
                attrs,
                dflt_lang,
                kind.option,
                kind.null,
            )?,

            Type::U16 => json_impl.add_int_field::<u16>(
                field_ident,
                attrs,
                dflt_lang,
                kind.option,
                kind.null,
            )?,

            Type::U32 => json_impl.add_int_field::<u32>(
                field_ident,
                attrs,
                dflt_lang,
                kind.option,
                kind.null,
            )?,

            Type::U64 => json_impl.add_int_field::<u64>(
                field_ident,
                attrs,
                dflt_lang,
                kind.option,
                kind.null,
            )?,

            Type::U128 => json_impl.add_int_field::<u128>(
                field_ident,
                attrs,
                dflt_lang,
                kind.option,
                kind.null,
            )?,

            Type::USize => json_impl.add_int_field::<usize>(
                field_ident,
                attrs,
                dflt_lang,
                kind.option,
                kind.null,
            )?,

            Type::String => {
                json_impl.add_string_field(field_ident, attrs, dflt_lang, kind.option, kind.null)?
            }

            Type::F32 => json_impl.add_float_field::<f32>(
                field_ident,
                attrs,
                dflt_lang,
                kind.option,
                kind.null,
            )?,

            Type::F64 => json_impl.add_float_field::<f64>(
                field_ident,
                attrs,
                dflt_lang,
                kind.option,
                kind.null,
            )?,

            Type::Bool => {
                json_impl.add_bool_field(field_ident, attrs, dflt_lang, kind.option, kind.null)?;
            }

            Type::Vec(ty, of) => {
                json_impl.add_vec_field(
                    ty,
                    field_ident,
                    attrs,
                    dflt_lang,
                    kind.option,
                    kind.null,
                    *of,
                )?;
            }

            Type::Custom(ty) => {
                json_impl.add_custom_field(
                    field_ident,
                    attrs,
                    ty,
                    dflt_lang,
                    kind.option,
                    kind.null,
                )?;
            }
        };
    }

    let mut json_impl = json_impl.construct(struct_ident, generics);

    json_impl.append(quote! {
        impl ::from::json::FromJson for #struct_ident #generics {}
    });

    Ok((struct_fields, json_impl))
}
