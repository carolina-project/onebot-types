use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, token::Comma, ItemStruct, LitStr, Type};

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

#[proc_macro_attribute]
pub fn onebot_action(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_struct: ItemStruct = parse_macro_input!(input);
    let args = parse_macro_input!(args as OBActionArgs);
    let action_name = args.action_name.value();
    let resp_type = args.response_type;

    let struct_name = &input_struct.ident;
    TokenStream::from(quote! {
        #[cfg_attr(
            not(target_arch = "wasm32"),
            derive(serde::Deserialize, serde::Serialize),
        )]
        #input_struct

        impl ob_types_base::OBAction for #struct_name {
            type Resp = #resp_type;

            fn action(&self) -> &str {
                #action_name
            }
        }
    })
}

#[proc_macro_attribute]
pub fn native(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let input: proc_macro2::TokenStream = input.into();
    let attrs: proc_macro2::TokenStream = attrs.into();
    quote! {
        #[cfg_attr(
            not(target_arch = "wasm32"),
            #attrs
        )]
        #input
    }
    .into()
}

#[proc_macro_attribute]
pub fn native_data(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let input: proc_macro2::TokenStream = input.into();
    let attrs: proc_macro2::TokenStream = attrs.into();
    quote! {
        #[cfg_attr(
            not(target_arch = "wasm32"),
            derive(serde::Deserialize, serde::Serialize),
            #attrs
        )]
        #input
    }
    .into()
}
