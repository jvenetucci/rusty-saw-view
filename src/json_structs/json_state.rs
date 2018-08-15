// Joseph Venetucci

//! `json_state` is a collection of structures that represent the JSON structured 
//! data from the `/state` endpoint of Hyperledger Sawtooth. Also includes 
//! methods for retrieving useful information and displaying these structures.
//! 
//! Typically when you parse JSON data from the `/state` endpoint you'll store it in the root
//! [StateData](struct.StateData.html) structure. That structure contains methods for printing out
//! the contents of the blockchain. Use [display_full_data](struct.StateData.html#method.display_full_data) when 
//! printing to the terminal, and [display_full_data_no_color](struct.StateData.html#method.display_full_data_no_color)
//! if you want to pipe the output to a file.

use super::json_blocks::{Paging};
use json_deserialize::*;
use super::{get_partial_string};
use colored::*;

/// A structure that represents the root data item found at the `/state` endpoint.
#[derive(Deserialize, Debug, Default)]
pub struct StateData {
    data: Vec<State>,
    head: String,
    link: String,
    paging: Paging
}

impl StateData {

    /// Returns the number of addresses that contain state.
    pub fn get_num_states(&self) -> usize {
        self.data.len()
    }

    /// Display the addresses that contain state, and the state stored.
    /// Uses colored text so use this for terminal printing.
    /// 
    /// If instead you want to print this to a terminal window, see [display_full_data](struct.StateData.html#method.display_full_data)
    /// since it prints with colored text.
    /// 
    /// The function takes in a tuple (full_id, show_settings, method) of settings:
    /// - `full_id` -> Setting this to false will only print the first 6 and last 4 characters of addresses, 
    ///     otherwise the full address is shown
    /// - `show_settings` -> Setting this to false will omit displaying address that contains blockchain setting data. It is
    ///     recommended that this stay false as its serialization will often be different from the rest of the data.
    /// - `method` -> What method to use when deserializing data. See
    ///     [supported methods](../../index.html#supported-deserialization-schemes) for a list of valid options
    pub fn display_full_data(self, (full_id, show_settings, method): (bool, bool, String)) {
        for state in self.data.iter() {
            if show_settings || state.get_address_namespace() != "000000" {
                if full_id {
                    println!("{} {}", "State Address:".green().on_black(), state.get_address_full());
                } else {
                    println!("{} {}", "State Address:".green().on_black(), get_partial_string(state.get_address_full(), 6, 4));
                }

                let payload_encoded = String::from(state.data.as_str());
                match method.as_str() {
                    "cbor" => println!("\tData:\n{}", parse_cbor(payload_encoded, 2).blue()),
                    _ => panic!("Unsupported deserialization method: {}", method)
                }
            }
        }
    }

    /// Display the addresses that contain state, and the state stored.
    /// No terminal color, so use this for piping to a file.
    /// 
    /// If instead you want to print this to a terminal window, see [display_full_data](struct.StateData.html#method.display_full_data)
    /// since it prints with colored text.
    /// 
    /// The function takes in a tuple (full_id, show_settings, method) of settings:
    /// - `full_id` -> Setting this to false will only print the first 6 and last 4 characters of addresses, 
    ///     otherwise the full address is shown
    /// - `show_settings` -> Setting this to false will omit displaying address that contains blockchain setting data. It is
    ///     recommended that this stay false as its serialization will often be different from the rest of the data.
    /// - `method` -> What method to use when deserializing data. See
    ///     [supported methods](../../index.html#supported-deserialization-schemes) for a list of valid options
    pub fn display_full_data_no_color(self, (full_id, show_settings, method): (bool, bool, String)) {
        for state in self.data.iter() {
            if show_settings || state.get_address_namespace() != "000000" {
                if full_id {
                    println!("{} {}", "State Address:", state.get_address_full());
                } else {
                    println!("{} {}", "State Address:", get_partial_string(state.get_address_full(), 6, 4));
                }

                let payload_encoded = String::from(state.data.as_str());
                match method.as_str() {
                    "cbor" => println!("\tData:\n{}", parse_cbor(payload_encoded, 2)),
                    _ => panic!("Unsupported deserialization method: {}", method)
                }
            }
        }
    }
}

/// A structure that represents a state found at the `/state` endpoint. States contain an address and the data stored at that address
#[derive(Deserialize, Debug, Default)]
pub struct State {
    address: String,
    data: String,
}

impl State {
    
    /// Returns the address namespace (i.e. the first 6 characters of the address). This will panic
    /// if the address is not a valid length of 70 characters.
    pub fn get_address_namespace(&self) -> String {
        match self.address.len() {
            70 => String::from(&self.address[0..6]), 
            _ => panic!("Invalid address"),
        }
    }

    /// Returns the full address
    pub fn get_address_full(&self) -> String {
        String::from(&self.address[0..])
    }
}

#[cfg(test)]
mod test_statedata_struct {
    use super::*;

    #[test]
    fn empty_has_zero_states() {
        let data = StateData::default();
        assert_eq!(0, data.get_num_states());
    }

    #[test]
    fn one_has_one_state() {
        let mut data = StateData::default();
        data.data.push(State::default());
        assert_eq!(1, data.get_num_states());
    }

    #[test]
    fn two_has_two_states() {
        let mut data = StateData::default();
        data.data.push(State::default());
        data.data.push(State::default());
        assert_eq!(2, data.get_num_states());
    }
}

#[cfg(test)]
mod test_state_struct {
    use super::*;

    #[test]
    fn namespace_is_six_characters_long() {
        let mut data = State::default();
        data.address = String::from("1cf126e83dbe4cdd233ab6402f1c19b0d93543f5da490356beab9c53435eef849dfcab");
        assert_eq!(6, data.get_address_namespace().len());
    }

    #[test]
    fn namespace_is_first_six_characters() {
        let mut data = State::default();
        data.address = String::from("1cf126e83dbe4cdd233ab6402f1c19b0d93543f5da490356beab9c53435eef849dfcab");
        assert_eq!(String::from("1cf126"), data.get_address_namespace());
    }

    #[test]
    #[should_panic(expected = "Invalid address")]
    fn invalid_namespace_panics() {
        let mut data = State::default();
        data.address = String::from("123ABC");
        data.get_address_namespace();
    }
}