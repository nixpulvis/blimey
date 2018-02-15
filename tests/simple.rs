#![feature(proc_macro)]
#![allow(dead_code)]

extern crate blimey;

use blimey::{Contract, contract, contractual};

// Create contracts.
const ANY_C: Contract = contract!(&|_| true);
const BOOLEAN_C: Contract = contract!(&|v| v.downcast_ref::<bool>().is_some());
const INTEGER_C: Contract = contract!(&|_| false);  // TODO: try a number of downcasts.
// const FUNC_C: Contract = contract!(INTEGER_C BOOLEAN_C -> INTEGER_C);
//
// // NOTE: Dependent contracts would be sexy as hell.
// const IDENTITY_C: Contract = contract!(ANY_C -> &|i| &|o| i == o)

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
#[contractual(contract!(INTEGER_C -> INTEGER_C))]
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

