#![feature(proc_macro)]

extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;

use proc_macro::{TokenStream, TokenNode, Delimiter, Literal};
use syn::*;
use syn::synom::*;
use quote::ToTokens;

// TODO: Use a real repr, not a fucking string.
#[derive(Debug)]
struct Contract(String);

impl Synom for Contract {
    named!(parse -> Self, value!(Contract("TODO".into())));
}

#[proc_macro_attribute]
pub fn contract(args: TokenStream, input: TokenStream) -> TokenStream {
    let args: Contract = syn::parse(args).unwrap();
    println!("{:?}", args);
    let input: ItemFn = syn::parse(input).unwrap();
    quote!(#input).into()
}
