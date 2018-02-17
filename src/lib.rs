#![feature(proc_macro, box_syntax)]

extern crate blimey_macros;

use std::any::Any;

pub enum FlatContract<'a> {
    // NOTE: Maybe `T` should be an `Any` type allowing us to avoid the `Eq`
    // bound. Then in `check` we can downcast to all the types we want to
    // support, including regex matching.
    Base(Box<Any>),
    Predicate(&'a Fn(&Box<Any>) -> bool),
}

impl<'a> FlatContract<'a> {
    fn check(&self, value: &Box<Any>) -> bool {
        match *self {
            FlatContract::Base(ref v) => {
                match (v.downcast_ref::<i32>(), value.downcast_ref::<i32>()) {
                    (Some(a), Some(b)) => a == b,
                    _ => false,
                }
            },
            FlatContract::Predicate(f) => f(value),
        }
    }
}

pub enum Contract<'a> {
    Flat(FlatContract<'a>),
    Function(Box<Contract<'a>>, Box<Contract<'a>>),
}

impl<'a> Contract<'a> {
    fn check(&self, value: &Box<Any>) -> bool {
        match *self {
            Contract::Flat(ref c) => c.check(value),
            Contract::Function(ref d, ref r) => {
                // TODO: Function contracts really need to wrap the function
                // too... we need to have access to the arguments to the
                // function to check. Here `value` is the function. `d` is the
                // domain function and `r` is the range contract.
                // I want to write something along the lines of
                // ````rust
                // |v| {
                //     d.check(v);
                //     let o = value(v);
                //     r.check(o);
                //     o
                // }
                // ```
                false
            }
        }
    }
}

pub struct Monitor<'a>(Box<Any>, &'a Contract<'a>);

impl<'a> std::ops::Deref for Monitor<'a> {
    type Target = Box<Any>;

    fn deref(&self) -> &Self::Target {
        if self.1.check(&self.0) {
            &self.0
        } else {
            // TODO: Blame.
            panic!("wtf");
        }
    }
}

pub use blimey_macros::{contract, contractual};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_contract_pass() {
        let one_c = Contract::Flat(FlatContract::Base(box 1));
        let one = Monitor(box 1, &one_c);
        assert_eq!(1, *one.downcast_ref::<i32>().unwrap());
    }

    #[test]
    #[should_panic]
    fn base_contract_fail() {
        let one_c = Contract::Flat(FlatContract::Base(box 1));
        let zero = Monitor(box 0, &one_c);
        assert_eq!(0, *zero.downcast_ref::<i32>().unwrap());
    }

    #[test]
    fn predicate_contract_pass() {
        let f = |ref v: &Box<Any>| {
            v.downcast_ref::<i32>().map_or(false, |i| *i > 0)
        };
        let count_c = Contract::Flat(FlatContract::Predicate(&f));
        let one = Monitor(box 1, &count_c);
        assert_eq!(1, *one.downcast_ref::<i32>().unwrap());
    }

    #[test]
    #[should_panic]
    fn predicate_contract_fail() {
        let f = |ref v: &Box<Any>| {
            v.downcast_ref::<i32>().map_or(false, |i| *i > 0)
        };
        let count_c = Contract::Flat(FlatContract::Predicate(&f));
        let zero = Monitor(box 0, &count_c);
        assert_eq!(0, *zero.downcast_ref::<i32>().unwrap());
    }

    #[test]
    fn function_contract_pass() {
        let one_c1 = Contract::Flat(FlatContract::Base(box 1));
        let one_c2 = Contract::Flat(FlatContract::Base(box 1));
        let fun_c = Contract::Function(box one_c1, box one_c2);
        let id = Monitor(box ((|v: i32| v) as fn(i32) -> i32), &fun_c);
        assert_eq!(1, id.downcast_ref::<fn(i32) -> i32>().unwrap()(1));
    }

    #[test]
    #[should_panic]
    fn function_contract_domain_fail() {
        let one_c1 = Contract::Flat(FlatContract::Base(box 1));
        let one_c2 = Contract::Flat(FlatContract::Base(box 1));
        let fun_c = Contract::Function(box one_c1, box one_c2);
        let id = Monitor(box ((|v: i32| v) as fn(i32) -> i32), &fun_c);
        assert_eq!(0, id.downcast_ref::<fn(i32) -> i32>().unwrap()(0));
    }

    #[test]
    #[should_panic]
    fn function_contract_range_fail() {
        let one_c1 = Contract::Flat(FlatContract::Base(box 1));
        let one_c2 = Contract::Flat(FlatContract::Base(box 1));
        let fun_c = Contract::Function(box one_c1, box one_c2);
        let id = Monitor(box ((|v: i32| v + 1) as fn(i32) -> i32), &fun_c);
        assert_eq!(2, id.downcast_ref::<fn(i32) -> i32>().unwrap()(1));
    }
}
