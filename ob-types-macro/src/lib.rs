use std::{fmt::Display, fs::OpenOptions};

use parse::Parse;
use proc::{JsonAddition, JsonProcMacro};
use proc_macro::TokenStream;
use quote::quote;
use syn::*;
use token::Comma;

mod proc;

struct OBActionArgs {
    action_name: LitStr,
    _comma: Comma,
    response_type: Type,
}
impl Parse for OBActionArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            action_name: input.parse()?,
            _comma: input.parse()?,
            response_type: input.parse()?,
        })
    }
}

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

#[proc_macro_derive(OBRespData)]
pub fn ob_resp_data(input: TokenStream) -> TokenStream {
    use syn::DeriveInput;
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let generics = &input.generics;
    let generic_types: Vec<_> = generics.type_params().map(|ty| &ty.ident).collect();
    let const_generics: Vec<_> = generics.const_params().map(|ty| &ty.ident).collect();
    let where_clause = &generics.where_clause;
    let lifetimes = generics.lifetimes();
    let t = quote! {
        #[cfg(not(feature = "json"))]
        impl #generics ob_types_base::OBRespData for #struct_name < #(#lifetimes,)*
            #(#generic_types, )* #(#const_generics, )*
            > #where_clause {}
    };
    t.into()
}

#[proc_macro_attribute]
pub fn onebot_action(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_struct: ItemStruct = parse_macro_input!(input);
    let args = parse_macro_input!(args as OBActionArgs);
    let action_name = args.action_name.value();
    let resp_type = args.response_type;

    let struct_name = &input_struct.ident;
    TokenStream::from(quote! {
        #[ob_types_macro::json]
        #input_struct

        impl #struct_name {
            pub const ACTION: &'static str = #action_name;
        }

        impl ob_types_base::OBAction for #struct_name {
            type Resp = #resp_type;

            fn action(&self) -> &str {
                #struct_name::ACTION
            }
        }
    })
}

#[proc_macro_attribute]
pub fn json(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let props: JsonProcMacro = parse_macro_input!(attrs);
    let attrs = props.inner;

    let derive = parse_macro_input!(input as DeriveInput);
    let input = match props.addition {
        Some(JsonAddition::StringValue) => {
            proc::derive_serde_process(derive, Some(Box::new(proc::str_field_append)))
        }
        Some(JsonAddition::OBRespDerive) => {
            let inp = proc::derive_serde_process(derive, None);
            quote! {
                #[derive(ob_types_macro::OBRespData)]
                #inp
            }
        }
        None => proc::derive_serde_process(derive, None),
    };

    let tokens = quote! {
        #[cfg_attr(
            feature = "json",
            derive(serde::Deserialize, serde::Serialize),
            #attrs
        )]
        #[derive(Debug, Clone)]
        #input
    };
    tokens.into()
}
