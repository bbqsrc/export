extern crate proc_macro;

use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn unstable(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as proc_macro2::TokenStream);
    let item = parse_macro_input!(item as proc_macro2::TokenStream);
    
    match export_macro::unstable(attr, item) {
        Ok(tokens) => tokens.into(),
        Err(err) => TokenStream::from(err.to_compile_error()),
    }
}
