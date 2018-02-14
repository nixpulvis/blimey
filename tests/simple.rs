#![feature(proc_macro)]

extern crate blimey_macros;

#[cfg(test)]
mod tests {
    use blimey_macros::contract;

    // A very simple function.
    // #[contract = n > 0 -> > 1]
    #[contract]
    fn add1(n: u64) -> u64 {
        n + 1
    }

    #[test]
    fn simple_test() {
        assert_eq!(1, add1(0));
    }
}

