use std::{fmt::Display, fs::OpenOptions};

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::Parse, parse_macro_input, parse_quote, token::Comma, Attribute, Data, DataEnum,
    DataStruct, DeriveInput, Field, Fields, Generics, Ident, ItemStruct, LitStr, Type, Visibility,
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

static FROMSTR_TYPES: [&str; 10] = [
    "u8", "u16", "u32", "u64", "i8", "i16", "i32", "i64", "f32", "f64",
];

/// Process enum fields with serde attributes, wrap fields with `serde` attribute in `cfg_attr` attribute, make sure attribute only enabled with `json` feature.
fn attrs_proc(attrs: &[Attribute]) -> Vec<Attribute> {
    attrs
        .into_iter()
        .map(|a| {
            if a.path()
                .get_ident()
                .map(|i| i == "serde")
                .unwrap_or_default()
            {
                let meta = &a.meta;
                parse_quote! {
                    #[cfg_attr(
                        feature = "json",
                        #meta
                    )]
                }
            } else {
                a.clone()
            }
        })
        .collect()
}

/// Append serde attributes to enum fields, making a field can be converted from a string.
fn str_field_append(field: &Field) -> Vec<Attribute> {
    let field_type = &field.ty;
    let mut attrs = vec![];
    if let Type::Path(typ) = field_type {
        if FROMSTR_TYPES.iter().any(|r| typ.path.is_ident(r)) {
            attrs.push(parse_quote! {
                #[serde(with = "ob_types_base::tool::from_json_str")]
            });
        } else if typ.path.is_ident("bool") {
            attrs.push(parse_quote! {
                #[serde(with = "ob_types_base::tool::str_bool")]
            });
        }
    }
    attrs
}

/// Process enum fields with Serde attributes, append attributes for fields and variants, and wrap fields with the `serde` attribute in a `cfg_attr` attribute to ensure that the attribute is only enabled with the `json` feature.
fn enum_fields_process(
    vis: Visibility,
    name: Ident,
    generics: Generics,
    data: DataEnum,
    extra_attrs_getter: Option<Box<dyn Fn(&Field) -> Vec<Attribute>>>,
    proc_fn: Option<Box<dyn Fn(&Field) -> Field>>,
) -> proc_macro2::TokenStream {
    let vars = data.variants.into_iter().map(|v| {
        let v_name = v.ident;
        let attrs = attrs_proc(&v.attrs);

        let field_proc = |mut def: Field| {
            if let Some(f) = extra_attrs_getter.as_ref() {
                def.attrs.extend(f(&def));
            }
            def.attrs = attrs_proc(&def.attrs);
            if let Some(f) = proc_fn.as_ref() {
                f(&def)
            } else {
                def
            }
        };

        let v_fields = match v.fields {
            Fields::Unit => quote! {},
            Fields::Named(fields) => {
                let field_defs = fields.named.into_iter().map(field_proc);
                quote! {
                    { #(#field_defs),* }
                }
            }
            Fields::Unnamed(fields) => {
                let field_defs = fields.unnamed.into_iter().map(field_proc);
                quote! {
                    ( #(#field_defs),* )
                }
            }
        };
        quote! {
            #(#attrs)* #v_name #v_fields
        }
    });
    quote! {
        #vis enum #name #generics {
            #(#vars),*
        }
    }
}

/// Process struct fields with Serde attributes, append attributes for fields, and wrap fields with the `serde` attribute in a `cfg_attr` attribute to ensure that the attribute is only enabled with the `json` feature.
fn struct_fields_proc(
    vis: Visibility,
    name: Ident,
    generics: Generics,
    data: DataStruct,
    extra_attrs_getter: Option<Box<dyn Fn(&Field) -> Vec<Attribute>>>,
    proc_fn: Option<Box<dyn Fn(&Field) -> Field>>,
) -> proc_macro2::TokenStream {
    let field_proc = |mut def: Field| {
        if let Some(f) = extra_attrs_getter.as_ref() {
            def.attrs.extend(f(&def));
        }
        def.attrs = attrs_proc(&def.attrs);
        if let Some(f) = proc_fn.as_ref() {
            f(&def)
        } else {
            def
        }
    };
    let field_defs = data.fields.into_iter().map(field_proc);

    quote! {
        #vis struct #name #generics {
            #(#field_defs),*
        }
    }
}

fn derive_serde_process(
    input: DeriveInput,
    extra_attrs_getter: Option<Box<dyn Fn(&Field) -> Vec<Attribute>>>,
) -> proc_macro2::TokenStream {
    let attrs = input.attrs;
    let data = match input.data {
        Data::Struct(data) => struct_fields_proc(
            input.vis,
            input.ident,
            input.generics,
            data,
            extra_attrs_getter,
            None::<_>,
        ),
        Data::Enum(data) => enum_fields_process(
            input.vis,
            input.ident,
            input.generics,
            data,
            extra_attrs_getter,
            None::<_>,
        ),
        _ => panic!("Only be derived for structs and enums."),
    };
    quote! {
        #(#attrs)*
        #data
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
    let derive = parse_macro_input!(input as DeriveInput);
    let input = derive_serde_process(derive, Some(Box::new(str_field_append)));
    let attrs: proc_macro2::TokenStream = attrs.into();
    let tokens = quote! {
        #[cfg_attr(
            feature = "json",
            derive(serde::Deserialize, serde::Serialize),
            #attrs
        )]
        #[derive(ob_types_macro::OBRespData, Debug, Clone)]
        #input
    };
    tokens.into()
}

#[proc_macro_attribute]
pub fn json(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let attrs: proc_macro2::TokenStream = attrs.into();
    let derive = parse_macro_input!(input as DeriveInput);
    let input = derive_serde_process(derive, None);
    let tokens = quote! {
        #[cfg_attr(
            feature = "json",
            derive(serde::Deserialize, serde::Serialize),
            #attrs
        )]
        #[derive(ob_types_macro::OBRespData, Debug, Clone)]
        #input
    };
    tokens.into()
}
