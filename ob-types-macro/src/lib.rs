use std::{fmt::Display, fs::OpenOptions};

use parse::Parse;
use proc::{JsonAddition, JsonProcMacro};
use proc_macro::TokenStream;
use quote::quote;
use syn::*;

mod proc;

struct OBActionArgs {
    response_type: Type,
}
impl Parse for OBActionArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
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
    let action_name = proc::camel_to_snake(&input_struct.ident.to_string());
    let resp_type = args.response_type;

    let struct_name = &input_struct.ident;
    TokenStream::from(quote! {
        #[ob_types_macro::data]
        #input_struct

        impl ob_types_base::OBAction<'static> for #struct_name {
            const ACTION: Option<&'static str> = Some(#action_name);
            type Resp = #resp_type;
        }
    })
}

#[proc_macro_attribute]
pub fn data(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let props: JsonProcMacro = parse_macro_input!(attrs);

    let derive = parse_macro_input!(input as DeriveInput);
    let mut input = if props.additions.contains(&JsonAddition::StringValue) {
        proc::derive_serde_process(derive, Some(Box::new(proc::str_field_append)))
    } else {
        proc::derive_serde_process(derive, None)
    };

    if props.has_default {
        input = quote! {
            #[derive(Default)]
            #input
        }
    }

    let tokens = quote! {
        #[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
        #input
    };
    tokens.into()
}
