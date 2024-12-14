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
#[proc_macro_derive(OBAction, attributes(resp, __oba_crate_path))]
pub fn onebot_action(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);

    let mut resp_type = None;
    let mut crate_path: Path = parse_quote! { ::onebot_types };
    for attr in &input.attrs {
        if attr.path().is_ident("resp") {
            let ty: Path = attr.parse_args().unwrap();
            resp_type = Some(ty);
        } else if attr.path().is_ident("__oba_crate_path") {
            crate_path = attr.parse_args().unwrap();
        }
    }
    if resp_type.is_none() {
        proc_macro_error::abort!(input, "The attribute `#[resp(<Type>)]` must be specified.")
    }

    let name = input.ident;
    let action_name = proc::camel_to_snake(&name.to_string());
    TokenStream::from(quote! {
        impl #crate_path::OBAction for #name {
            const ACTION: Option<&'static str> = Some(#action_name);
            type Resp = #resp_type;
        }
    })
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
