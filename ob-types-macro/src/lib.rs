use proc::{selector, DeriveAttr, __data};
use proc_macro::TokenStream;
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

fn process_onebot_action(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let in_name = input.ident.clone();
    let attrs = DeriveAttr::<Path>::parse("action", &input.attrs, &in_name, Some("resp"))?;

    let full_name = attrs.full_name();
    let custom_attr = attrs.custom_attr.ok_or_else(|| {
        syn::Error::new_spanned(
            input,
            "The response type must be specified.(`#[action(resp = <Type>)]`)",
        )
    })?;
    let crate_path = attrs.crate_path;
    Ok(quote! {
        impl #crate_path::OBAction for #in_name {
            const ACTION: Option<&'static str> = Some(#full_name);
            type Resp = #custom_attr;
        }
    })
}

#[proc_macro_derive(OBAction, attributes(action))]
pub fn onebot_action(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);

    process_onebot_action(input)
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}

fn process_msg_seg(input: DeriveInput) -> Result<proc_macro2::TokenStream> {
    let in_name = input.ident.clone();
    let attrs = DeriveAttr::<Path>::parse("msg", &input.attrs, &in_name, None)?;

    let full_name = attrs.full_name();
    let crate_path = attrs.crate_path;
    Ok(quote! {
        impl #crate_path::OBMessage for #in_name {
            const TYPE: &'static str = #full_name;
        }
    })
}

#[proc_macro_derive(OBMessage, attributes(msg))]
pub fn onebot_message(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);

    process_msg_seg(input)
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}

fn process_onebot_event(input: DeriveInput) -> Result<proc_macro2::TokenStream> {
    let in_name = input.ident.clone();

    let attrs = DeriveAttr::<LitStr>::parse("event", &input.attrs, &in_name, Some("type"))?;

    let full_name = attrs.full_name();
    let custom_attr = attrs.custom_attr.ok_or_else(|| {
        syn::Error::new_spanned(
            input,
            "The event type must be specified.(`#[event(type = \"<Type(message, notice ...)>\")]`)",
        )
    })?;
    let crate_path = attrs.crate_path;
    Ok(quote! {
        impl #crate_path::OBEvent for #in_name {
            const TYPE: &'static str = #custom_attr;
            const DETAIL_TYPE: &'static str = #full_name;
        }
    })
}

#[proc_macro_derive(OBEvent, attributes(event))]
pub fn onebot_event(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);

    process_onebot_event(input)
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}

#[cfg(feature = "ob12")]
#[proc_macro_derive(OBEventSelector, attributes(selector))]
pub fn onebot_event_selector(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);

    selector::gen_selector(input)
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
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
        __data::derive_serde_process(derive, Some(Box::new(__data::str_field_append)))
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
        #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
        #input
    }
    .into()
}
