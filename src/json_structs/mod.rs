pub mod json_blocks;
pub mod json_state;

/// Returns a string of the first n and last m characters of the given string
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