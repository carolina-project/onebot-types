use parse::Parse;
use quote::quote;
use syn::*;

static FROMSTR_TYPES: [&str; 10] = [
    "u8", "u16", "u32", "u64", "i8", "i16", "i32", "i64", "f32", "f64",
];

/// Append serde attributes to enum fields, making a field can be converted from a string.
pub fn str_field_append(field: &Field) -> Vec<Attribute> {
    let field_type = &field.ty;
    let mut attrs = vec![];
    if let Type::Path(typ) = field_type {
        if FROMSTR_TYPES.iter().any(|r| typ.path.is_ident(r)) {
            attrs.push(parse_quote! {
                #[serde(with = "crate::base::tool::from_str")]
            });
        } else if typ.path.is_ident("bool") {
            attrs.push(parse_quote! {
                #[serde(with = "crate::base::tool::str_bool")]
            });
        }
    }
    attrs
}

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
        let attrs = v.attrs;

        let field_proc = |mut def: Field| {
            if let Some(f) = extra_attrs_getter.as_ref() {
                def.attrs.extend(f(&def));
            }
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

pub fn derive_serde_process(
    input: DeriveInput,
    extra_attrs_getter: Option<Box<dyn Fn(&Field) -> Vec<Attribute>>>,
) -> proc_macro2::TokenStream {
    let attrs = &input.attrs;
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

pub(crate) struct DeriveAttr<T: Parse> {
    pub imp: Option<syn::LitStr>,
    pub name: syn::LitStr,
    pub crate_path: syn::Path,
    pub custom_attr: Option<T>,
}

impl<T: Parse> DeriveAttr<T> {
    pub fn parse(
        attr_name: &str,
        attrs: &[syn::Attribute],
        name: &Ident,
        custom_attr_name: &str,
    ) -> Result<Self> {
        let mut custom = None::<T>;
        let mut crate_path: Path = parse_quote! { ::onebot_types };
        let mut imp = None::<syn::LitStr>;
        let mut name = LitStr::new(&camel_to_snake(&name.to_string()), proc_macro2::Span::call_site());
        for attr in attrs {
            if attr.path().is_ident(attr_name) {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident(custom_attr_name) {
                        custom = Some(meta.value()?.parse()?);
                    } else if meta.path.is_ident("__crate_path") {
                        crate_path = meta.value()?.parse()?;
                    } else if meta.path.is_ident("imp") {
                        imp = meta.value()?.parse()?;
                    } else if meta.path.is_ident("rename") {
                        name = meta.value()?.parse()?;
                    }

                    Ok(())
                })?;
            }
        }

        Ok(Self {
            imp,
            name,
            crate_path,
            custom_attr: custom,
        })
    }

    pub fn full_name(&self) -> String {
        if let Some(imp) = &self.imp {
            format!("{}.{}", imp.value(), self.name.value())
        } else {
            self.name.value()
        }
    }
}

fn filter_enum_variants(variants: syn::punctuated::IntoIter<Variant>) -> Result<(Vec<Ident>, Vec<Field>)> {
    let mut names = vec![];
    let mut fields_vec = vec![];
    for var in variants {
        match var.fields {
            Fields::Named(_) => {
                return Err(syn::Error::new_spanned(var, "expected unnamed variant"))
            },
            Fields::Unnamed(fields) => {
                let mut fields_: Vec<_> = fields.unnamed.clone().into_iter().collect();
                if fields_.len() > 1 {
                    return Err(syn::Error::new_spanned(fields, "more than 1 fields received"))
                }
                names.push(var.ident);
                fields_vec.push(fields_.remove(0));
            },
            Fields::Unit => 
                return Err(syn::Error::new_spanned(var, "expected unnamed variant")),
        }
    }

    Ok((names, fields_vec))
}

fn generate_impl(crate_path: Path, name: Ident, names: Vec<Ident>, fields: Vec<Field>) -> proc_macro2::TokenStream {
    quote! {
        impl #crate_path::OBEventSelector for #name {
            fn deserialize_event(event: #crate_path::ob12::event::EventDetail) -> Result<Self, DeserializerError> {
                use serde::de::IntoDeserializer;

                let #crate_path::ob12::event::EventDetail { r#type, detail_type, detail } = event;
                let event = match (r#type.as_str(), detail_type.as_str()) {
                    #( ( <#fields as #crate_path::OBEvent>::TYPE, <#fields as #crate_path::OBEvent>::DETAIL_TYPE ) => 
                            #name::#names( <#fields as serde::Deserialize>::deserialize(detail.into_deserializer())? ),
                    )*
                };
                Ok(event)
            }
        }
    }
}

pub fn gen_selector(data: DataEnum) -> Result<proc_macro2::TokenStream, Error> {
    let variants = data.variants.into_iter().filter(|v| v.fields.)
}
