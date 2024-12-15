use proc::{gen_selector, DeriveAttr};
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use punctuated::Punctuated;
use quote::{quote, ToTokens};
use std::{fmt::Display, fs::OpenOptions};
use syn::*;

mod proc;

#[allow(dead_code)]
fn debug_tokens(tokens: impl Display) {
    use std::io::Write;
    let mut f = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("/tmp/macro_debug")
        .unwrap();
    writeln!(f, "{}", tokens).unwrap();
}

#[proc_macro_error]
#[proc_macro_derive(OBAction, attributes(action))]
pub fn onebot_action(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    let in_name = input.ident.clone();

    let attrs = DeriveAttr::<Path>::parse("action", &input.attrs, &in_name, "resp").unwrap();
    if attrs.custom_attr.is_none() {
        proc_macro_error::abort!(
            input,
            "The response type must be specified.(`#[action(resp = <Type>)]`)"
        )
    }

    let full_name = attrs.full_name();
    let DeriveAttr {
        crate_path,
        custom_attr,
        ..
    } = attrs;
    TokenStream::from(quote! {
        impl #crate_path::OBAction for #in_name {
            const ACTION: Option<&'static str> = Some(#full_name);
            type Resp = #custom_attr;
        }
    })
}

#[proc_macro_error]
#[proc_macro_derive(OBEvent, attributes(event))]
pub fn onebot_event(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    let in_name = input.ident.clone();

    let attrs = DeriveAttr::<LitStr>::parse("event", &input.attrs, &in_name, "type").unwrap();
    if attrs.custom_attr.is_none() {
        proc_macro_error::abort!(
            input,
            "The event type must be specified.(`#[event(type = \"<Type(message, notice ...)>\")]`)"
        )
    }

    let full_name = attrs.full_name();
    let DeriveAttr {
        crate_path,
        custom_attr,
        ..
    } = attrs;
    quote! {
        impl #crate_path::OBEvent for #in_name {
            const TYPE: &'static str = #custom_attr;
            const DETAIL_TYPE: &'static str = #full_name;
        }
    }
    .into()
}

#[proc_macro_error]
#[proc_macro_derive(OBEventSelector)]
pub fn onebot_event_selector(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);

    let Data::Enum(data) = input.data.clone() else {
        proc_macro_error::abort!(input, "expected Enum")
    };
    gen_selector(data).unwrap().into()
}

#[proc_macro_attribute]
pub fn __data(attr: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr with Punctuated::<Meta, syn::Token![,]>::parse_terminated);
    let derive = parse_macro_input!(input as DeriveInput);

    let mut str_field = false;
    let mut default = false;
    for ele in args {
        if ele.path().is_ident("str") {
            str_field = true;
        } else if ele.path().is_ident("default") {
            default = true;
        }
    }

    let mut input = if str_field {
        proc::derive_serde_process(derive, Some(Box::new(proc::str_field_append)))
    } else {
        derive.into_token_stream()
    };
    if default {
        input = quote! {
            #[derive(Default)]
            #input
        };
    }

    quote! {
        #[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
        #input
    }
    .into()
}
