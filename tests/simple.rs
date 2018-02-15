#![feature(proc_macro)]

extern crate blimey;

use std::any::Any;
use blimey::{contract, contractual};

type Contract = &'static Fn(&Any) -> bool;

// Create contracts.
const boolean_c: Contract = contract!(&|v| v.downcast_ref::<bool>().is_some());
const integer_c: Contract = contract!(&|v| false);  // TODO: try a number of downcasts.
// const func_c: Contract = contract!(integer_c boolean_c -> integer_c);

// #[contractual]
// struct A;

#[contractual]
fn check_me() {
    // noop
}

fn dont_check_me() {
    // noop
}

// A very simple function.
#[contractual(contract!(integer_c -> integer_c))]
fn add1(n: u64) -> u64 {
    check_me();
    // TODO: Would be pretty awesome.
    // (#[contractual] || {})();
    dont_check_me();
    (|| {})();  // NOTE: Should not be checked.
    n + 1
}

#[test]
fn simple_test() {
    assert_eq!(1, add1(0));
}

