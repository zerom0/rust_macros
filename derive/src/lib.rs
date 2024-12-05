extern crate proc_macro;
use proc_macro::TokenStream;

use syn::{parse_macro_input, DeriveInput};
use quote::quote;


/// Example of user-defined [derive mode macro][1]
///
/// [1]: https://doc.rust-lang.org/reference/procedural-macros.html#derive-mode-macros
#[proc_macro_derive(Hello)]
pub fn hello(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);

    let tokens = quote! {
        impl Hello for #ident {
            fn say_hello(&self) {
                println!("Hello from {} my fellow rustaceans!", stringify!(#ident));
            }
        }
    };

    tokens.into()
}
