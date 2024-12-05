extern crate proc_macro;
use proc_macro::TokenStream;

//use syn::{parse_macro_input, DeriveInput};
use quote::quote;

/// Example of [function-like procedural macro][1].
///
/// [1]: https://doc.rust-lang.org/reference/procedural-macros.html#function-like-procedural-macros
#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    // let input = parse_macro_input!(input as DeriveInput);

    println!("{input:?}");

    let output = input
        .into_iter()
        .map(|t| {
            let mut s = t.to_string();
            if s == "<" {
                s = String::from("&lt;")
            }
            s
        })
        .collect::<Vec<String>>()
        .join(" ");

    let tokens = quote! {
         #output
    };

    tokens.into()
}

