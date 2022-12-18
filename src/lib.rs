// #![warn(missing_docs, unreachable_pub)]
// #![deny(unused_must_use, rust_2018_idioms)]
// #![doc(test(no_crate_inject, attr(deny(warnings, rust_2018_idioms)), allow(dead_code, unused_variables)))]
#![warn(clippy::all, clippy::pedantic, clippy::cargo, clippy::nursery)]

pub mod helpers;
pub mod errors;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
