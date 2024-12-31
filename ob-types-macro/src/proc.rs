use parse::Parse;
use quote::quote;
use syn::*;

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

pub(crate) mod __data {
    use super::*;

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

    type EAttrGetter = Option<Box<dyn Fn(&Field) -> Vec<Attribute>>>;
    type ProcFn = Option<Box<dyn Fn(&Field) -> Field>>;

    pub fn enum_fields_process(
        vis: Visibility,
        name: Ident,
        generics: Generics,
        data: DataEnum,
        extra_attrs_getter: EAttrGetter,
        proc_fn: ProcFn,
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
        extra_attrs_getter: EAttrGetter,
        proc_fn: ProcFn,
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
        let is_unit = matches!(data.fields, Fields::Unnamed(_));
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
        extra_attrs_getter: EAttrGetter,
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
        custom_attr_name: Option<&str>,
    ) -> Result<Self> {
        let mut custom = None::<T>;
        let mut crate_path: Path = parse_quote! { ::onebot_types };
        let mut imp = None::<syn::LitStr>;
        let mut name = LitStr::new(
            &camel_to_snake(&name.to_string()),
            proc_macro2::Span::call_site(),
        );
        for attr in attrs {
            if attr.path().is_ident(attr_name) {
                attr.parse_nested_meta(|meta| {
                    if let Some(custom_name) = custom_attr_name {
                        if meta.path.is_ident(custom_name) {
                            custom = Some(meta.value()?.parse()?);
                        }
                    };
                    if meta.path.is_ident("__crate_path") {
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

pub(crate) mod selector {
    use super::*;

    fn filter_enum_variants(
        variants: syn::punctuated::IntoIter<Variant>,
    ) -> Result<(Vec<Ident>, Vec<Field>)> {
        let mut names = vec![];
        let mut fields_vec = vec![];
        for var in variants {
            match var.fields {
                Fields::Named(_) => {
                    return Err(syn::Error::new_spanned(var, "expected unnamed variant"))
                }
                Fields::Unnamed(fields) => {
                    let mut fields_: Vec<_> = fields.unnamed.clone().into_iter().collect();
                    if fields_.len() > 1 {
                        return Err(syn::Error::new_spanned(
                            fields,
                            "more than 1 fields received",
                        ));
                    }
                    names.push(var.ident);
                    fields_vec.push(fields_.remove(0));
                }
                Fields::Unit => {
                    return Err(syn::Error::new_spanned(var, "expected unnamed variant"))
                }
            }
        }

        Ok((names, fields_vec))
    }

    fn generate_impl(
        crate_path: Path,
        name: Ident,
        names: Vec<Ident>,
        fields: Vec<Field>,
    ) -> proc_macro2::TokenStream {
        let selectable_var = Ident::new(
            &format!("__{}_SELECTABLE_SLICE__", name),
            proc_macro2::Span::call_site(),
        );
        let fields: Vec<_> = fields.into_iter().map(|r| r.ty).collect();

        let selectable = quote! {
            static #selectable_var: &'static [#crate_path::base::EventDesc] = &[
                #(
                    #crate_path::base::EventDesc {
                        r#type: <#fields as #crate_path::OBEvent>::TYPE,
                        detail_type: <#fields as #crate_path::OBEvent>::DETAIL_TYPE,
                    }
                ),*
            ];
        };

        let serde_impl = quote! {
            impl ::serde::Serialize for #name {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: ::serde::Serializer {
                    use #crate_path::OBEventSelector;

                    self
                        .serialize_event()
                        .map_err(::serde::ser::Error::custom)?
                        .serialize(serializer)
                }
            }

            impl<'de> ::serde::Deserialize<'de> for #name {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                    where
                        D: ::serde::Deserializer<'de> {
                    <Self as #crate_path::OBEventSelector>::deserialize_event(
                        #crate_path::ob12::event::EventDetail::deserialize(deserializer)?
                    ).map_err(serde::de::Error::custom)
                }
            }
        };

        quote! {
            #selectable

            impl #crate_path::OBEventSelector for #name {
                fn deserialize_event(
                    event: #crate_path::ob12::event::EventDetail
                ) -> Result<Self, ::serde_value::DeserializerError> {
                    use ::serde::{de::{IntoDeserializer, Error}, Deserialize};
                    use #crate_path::{ob12::event::EventDetail, OBEvent};

                    let EventDetail { r#type, detail_type, detail } = event;
                    let event = match (r#type.as_str(), detail_type.as_str()) {
                        #( ( <#fields as OBEvent>::TYPE, <#fields as OBEvent>::DETAIL_TYPE ) =>
                                #name::#names( <#fields as Deserialize>::deserialize(detail.into_deserializer())? ),
                        )*
                        (ty, det_ty)
                            => return Err(Error::custom(format!("unexpected event: {ty}.{det_ty}")))
                    };
                    Ok(event)
                }


                fn serialize_event(
                    &self
                ) -> Result<#crate_path::ob12::event::EventDetail, ::serde_value::SerializerError> {
                    use ::serde::{ser::Error, Deserialize};
                    use #crate_path::{OBEvent, ob12::event::EventDetail};

                    let (r#type, detail_type, detail) = match self {
                        #( #name::#names(detail) => (
                            <#fields as OBEvent>::TYPE.to_owned(),
                            <#fields as OBEvent>::DETAIL_TYPE.to_owned(),
                            Deserialize::deserialize(::serde_value::to_value(detail)?)
                                .map_err(Error::custom)?
                        ), )*
                    };

                    Ok(EventDetail { r#type, detail_type, detail })
                }


                fn get_selectable() -> &'static [#crate_path::base::EventDesc] {
                    #selectable_var
                }
            }

            #serde_impl
        }
    }

    pub fn gen_selector(input: DeriveInput) -> Result<proc_macro2::TokenStream> {
        let Data::Enum(data) = input.data else {
            return Err(syn::Error::new_spanned(input, "expected enum"));
        };

        let mut crate_path: Path = parse_quote! { ::onebot_types };
        for ele in input.attrs {
            if ele.path().is_ident("selector") {
                ele.parse_nested_meta(|meta| {
                    if meta.path.is_ident("__crate_path") {
                        crate_path = meta.value()?.parse()?;
                    }
                    Ok(())
                })?;
            }
        }
        let (names, fields) = filter_enum_variants(data.variants.into_iter())?;
        Ok(generate_impl(crate_path, input.ident, names, fields))
    }
}
