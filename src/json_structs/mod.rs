// Copyright (c) 2018 Joseph Venetucci
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE.md in the
// source distribution of this software for license terms.

//! `json_structs` is a collection of submodules that define structures that map to the JSON
//! data found at various endpoints for Hyperledger Sawtooth.
//! 
//! As of now there are only modules that represent the `/blocks` and `/state` endpoints.

pub mod json_blocks;
pub mod json_state;

/// Returns a string of the first *n* and last *m* characters of the given string
/// 
/// # Examples
///
/// ```
/// use json_structs::{get_partial_string}
///
/// let st = String::from("ABCddddABC");
/// assert_eq!(String::from("ABC...ABC"), get_partial_string(st, 3, 3));
/// ```
/// 
/// # Panic
/// This function will panic if n or m result in an operation that's out of bounds.
pub fn get_partial_string(full_string: String, n: usize, m: usize) -> String {
    if n > full_string.len() || m > full_string.len() {
        panic!("Invalid range for n/m")
    }
    format!("{}...{}", String::from(&full_string[0..n]),
        String::from(&full_string[(full_string.len() - m)..]))
}

#[cfg(test)]
mod test_partial_string {
    use super::*;

    #[test]
    fn first_3_last_3() {
        let st = String::from("ABCddddABC");
        assert_eq!(String::from("ABC...ABC"),get_partial_string(st, 3, 3));
    }

    #[test]
    fn first_3_last_0() {
        let st = String::from("ABCddddABC");
        assert_eq!(String::from("ABC..."),get_partial_string(st, 3, 0));
    }

    #[test]
    fn first_0_last_3() {
        let st = String::from("ABCddddABC");
        assert_eq!(String::from("...ABC"),get_partial_string(st, 0, 3));
    }

    #[test]
    fn small_string() {
        let st = String::from("ABC");
        assert_eq!(String::from("A...ABC"),get_partial_string(st, 1, 3));
    }

    #[test]
    #[should_panic(expected = "Invalid range")]
    fn small_string_n_out_of_bounds() {
        let st = String::from("ABC");
        get_partial_string(st, 4, 3);
    }

    #[test]
    #[should_panic(expected = "Invalid range")]
    fn small_string_m_out_of_bounds() {
        let st = String::from("ABC");
        get_partial_string(st, 1, 4);
    }
}