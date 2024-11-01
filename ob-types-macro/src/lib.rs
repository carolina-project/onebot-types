use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::Parse, parse_macro_input, token::Comma, Data, DataEnum, DataStruct, DeriveInput, Field,
    Fields, Generics, Ident, ItemStruct, LitStr, Type, Visibility,
};

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

#[proc_macro_derive(OBRespData)]
pub fn ob_resp_data(input: TokenStream) -> TokenStream {
    use syn::DeriveInput;
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let generics = &input.generics;
    let generic_types: Vec<_> = generics.type_params().map(|ty| &ty.ident).collect();
    let const_generics: Vec<_> = generics.const_params().map(|ty| &ty.ident).collect();
    let where_clause = &generics.where_clause;
    TokenStream::from(quote! {
        #[cfg(not(feature = "json"))]
        impl #generics ob_types_base::OBRespData for #struct_name <
            #(#generic_types, )* #(#const_generics, )*
            > #where_clause {}
    })
}

static FROMSTR_TYPES: [&str; 10] = [
    "u8", "u16", "u32", "u64", "i8", "i16", "i32", "i64", "f32", "f64",
];

fn struct_fields_proc(
    vis: Visibility,
    name: Ident,
    generics: Generics,
    data: DataStruct,
) -> proc_macro2::TokenStream {
    let field_defs = data.fields.iter().map(fields_iter);
    quote! {
        #vis struct #name #generics {
            #(#field_defs),*
        }
    }
}

fn fields_iter(field: &Field) -> proc_macro2::TokenStream {
    let field_type = &field.ty;
    if let Type::Path(typ) = field_type {
        if FROMSTR_TYPES.iter().any(|r| typ.path.is_ident(r)) {
            return quote! {
                #[cfg_attr(
                    feature = "json",
                    serde(with = "ob_types_base::tool::from_json_str")
                )]
                #field
            };
        } else if typ.path.is_ident("bool") {
            return quote! {
                #[cfg_attr(
                    feature = "json",
                    serde(with = "ob_types_base::tool::str_bool")
                )]
                #field
            };
        }
    }
    quote! {
        #field
    }
}

fn enum_fields_proc(
    vis: Visibility,
    name: Ident,
    generics: Generics,
    data: DataEnum,
) -> proc_macro2::TokenStream {
    let vars = data.variants.into_iter().map(|v| {
        let v_name = v.ident;
        let v_fields = match v.fields {
            Fields::Unit => quote! {},
            Fields::Named(fields) => {
                let field_defs = fields.named.iter().map(fields_iter);
                quote! {
                    { #(#field_defs),* }
                }
            }
            Fields::Unnamed(fields) => {
                let field_defs = fields.unnamed.iter().map(fields_iter);
                quote! {
                    ( #(#field_defs),* )
                }
            }
        };
        quote! {
            #v_name #v_fields
        }
    });

    quote! {
        #vis enum #name #generics {
            #(#vars),*
        }
    }
}

fn string_ser_deser(input: DeriveInput) -> proc_macro2::TokenStream {
    match input.data {
        Data::Struct(data) => struct_fields_proc(input.vis, input.ident, input.generics, data),
        Data::Enum(data) => enum_fields_proc(input.vis, input.ident, input.generics, data),
        _ => panic!("CustomSerde can only be derived for structs."),
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
            feature = "json",
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
pub fn json_from_str(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let attrs: proc_macro2::TokenStream = attrs.into();
    let derive = parse_macro_input!(input as DeriveInput);
    let input = string_ser_deser(derive);
    quote! {
        #[cfg_attr(
            feature = "json",
            derive(serde::Deserialize, serde::Serialize),
            #attrs
        )]
        #[derive(ob_types_macro::OBRespData, Debug)]
        #input
    }
    .into()
}

#[proc_macro_attribute]
pub fn json(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let attrs: proc_macro2::TokenStream = attrs.into();
    let input = parse_macro_input!(input as DeriveInput);
    quote! {
        #[cfg_attr(
            feature = "json",
            derive(serde::Deserialize, serde::Serialize),
            #attrs
        )]
        #[derive(ob_types_macro::OBRespData, Debug)]
        #input
    }
    .into()
}
