use proc_macro::TokenStream;
use quote::quote;

mod ast;
mod fmt;
mod parse;

#[proc_macro]
pub fn format_args(input: TokenStream) -> TokenStream {
    let template = syn::parse_macro_input!(input as ast::Template);
    let values = &template.values;

    let output = quote! {
        ::std::format_args!(#template, #(#values),*)
    };

    output.into()
}