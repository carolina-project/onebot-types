pub mod ob11_actions {
    use quote::*;
    use syn::*;

    /// Parse a source file, find types that implement `OBRespData`
    pub fn get_ob_actions(file: File) -> Vec<Ident> {
        file.items
            .into_iter()
            .filter_map(|item| {
                let Item::Struct(data) = item else {
                    return None;
                };

                if data
                    .attrs
                    .iter()
                    .any(|a| a.meta.path().is_ident("onebot_action"))
                {
                    Some(data.ident.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn make_enum(mods: Vec<(impl AsRef<str>, Vec<Ident>)>) -> proc_macro2::TokenStream {
        let variants: Vec<proc_macro2::TokenStream> = mods
            .iter()
            .map(|(mod_name, idents)| {
                let mod_name = mod_name.as_ref();
                idents.iter().map(move |ident| {
                    quote! {
                        #ident(#mod_name::#ident)
                    }
                })
            })
            .flatten()
            .collect();
        let idents: Vec<&Ident> = mods.iter().map(|(_, idents)| idents).flatten().collect();
        quote! {
            pub enum Action {
                #(#variants),*
            }

            impl Action {
                pub fn get_name(&self) -> &str {
                    match self {
                        #(
                            Action::#idents(_) => #idents::ACTION,
                        )*
                    }
                }
            }

            #[cfg(feature = "json")]
            impl Action {
                pub fn deserialize(
                    name: &str,
                    params: serde_json::Value,
                ) -> Result<Self, serde_json::Error> {
                    match name {
                        #(
                            #idents::ACTION => Ok(Action::#idents(serde_json::from_value(params)?)),
                        )*
                        _ => Err(serde_json::Error::custom("Unknown action")),
                    }
                }

                pub fn serialize_params(&self) -> serde_json::Result<serde_json::Value> {
                    match self {
                        #(
                            Action::#idents(data) => serde_json::to_value(data),
                        )*
                    }
                }
            }
        }
    }
}
