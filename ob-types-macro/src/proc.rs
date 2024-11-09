use std::collections::HashSet;

use parse::Parse;
use quote::quote;
use syn::*;

static FROMSTR_TYPES: [&str; 10] = [
    "u8", "u16", "u32", "u64", "i8", "i16", "i32", "i64", "f32", "f64",
];

/// Process enum fields with serde attributes, wrap fields with `serde` attribute in `cfg_attr` attribute, make sure attribute only enabled with `json` feature.
pub fn attrs_proc(attrs: &[Attribute]) -> Vec<Attribute> {
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
pub fn str_field_append(field: &Field) -> Vec<Attribute> {
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
pub fn enum_fields_process(
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
pub fn struct_fields_proc(
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
    let is_unit = match data.fields {
        Fields::Unnamed(_) => true,
        _ => false,
    };
    let field_defs = data.fields.into_iter().map(field_proc);

    if is_unit {
        quote! {
            #vis struct #name #generics (
                #( #field_defs ),*
            );
        }
    } else {
        quote! {
            #vis struct #name #generics {
                #( #field_defs ),*
            }
        }
    }
}

#[derive(Hash, PartialEq, Eq)]
pub enum JsonAddition {
    StringValue,
    OBRespDerive,
}

pub struct JsonProcMacro {
    pub additions: HashSet<JsonAddition>,
    pub has_default: bool,
    pub inner: proc_macro2::TokenStream,
}

impl Parse for JsonProcMacro {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        let mut additions = HashSet::new();
        let mut has_default = false;
        let peek = || -> Result<_> {
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }

            Ok(())
        };
        let mut inner: proc_macro2::TokenStream = quote! {};
        while input.peek(Ident) {
            let ident: Ident = input.parse()?;
            match ident.to_string().as_str() {
                "default" => {
                    if input.peek(Token![,]) {
                        input.parse::<Token![,]>()?;
                    }
                    has_default = true;
                }
                "str" => {
                    peek()?;
                    additions.insert(JsonAddition::StringValue);
                }
                "resp" => {
                    peek()?;
                    additions.insert(JsonAddition::OBRespDerive);
                }
                _ => {
                    let tokens: proc_macro2::TokenStream = input.parse()?;
                    inner = quote! { #ident #tokens };
                    break;
                }
            }
        }

        Ok(JsonProcMacro {
            additions,
            inner,
            has_default,
        })
    }
}

pub fn derive_serde_process(
    input: DeriveInput,
    extra_attrs_getter: Option<Box<dyn Fn(&Field) -> Vec<Attribute>>>,
) -> proc_macro2::TokenStream {
    let attrs = attrs_proc(&input.attrs);
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

pub fn camel_to_snake(s: &str) -> String {
    let mut result = String::new();
    let mut first = true;

    for c in s.chars() {
        if c.is_uppercase() {
            if !first {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
            first = false;
        } else {
            result.push(c);
        }
    }

    result
}
