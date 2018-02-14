#![feature(proc_macro)]

extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;

use proc_macro::{TokenStream, TokenNode, Delimiter, Literal};
use syn::token;
use quote::ToTokens;

#[proc_macro_attribute]
pub fn contract(args: TokenStream, input: TokenStream) -> TokenStream {
    let input: token::Fn = syn::parse(input).unwrap();
    quote!(#input).into()
}
