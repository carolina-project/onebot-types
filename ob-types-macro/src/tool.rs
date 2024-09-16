use proc_macro::TokenStream;
use quote::quote;

pub fn append_tokens(append: proc_macro2::TokenStream, input: TokenStream) -> TokenStream {
    let input: proc_macro2::TokenStream = input.into();
    quote! {
        #append
        #input
    }.into()
}
