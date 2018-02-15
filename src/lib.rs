#![feature(proc_macro)]

extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;

use proc_macro::{TokenStream, TokenNode, Delimiter, Literal};
use syn::*;
use syn::synom::*;
use syn::fold::{self, *};
use quote::ToTokens;

// TODO: Use a real repr, not a fucking string.
#[derive(Debug)]
struct Contract(String);

impl Synom for Contract {
    named!(parse -> Self, value!(Contract("TODO".into())));
}

struct Monitor;

impl Fold for Monitor {
    fn fold_expr(&mut self, e: Expr) -> Expr {
        match e {
            Expr::Call(e) => {
                println!("found a call to monitor!");
                Expr::Call(e)
            },
            _ => fold::fold_expr(self, e),
        }
    }
}

#[proc_macro_attribute]
pub fn contract(args: TokenStream, input: TokenStream) -> TokenStream {
    let args: Contract = syn::parse(args).unwrap();
    println!("{:?}", args);
    let mut input: ItemFn = syn::parse(input).unwrap();
    let output = Monitor.fold_item_fn(input);
    quote!(#output).into()
}
