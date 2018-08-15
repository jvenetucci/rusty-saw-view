//! `json_reader` contains methods for parsing blockchain JSON into structures found in [json_structs](../json_structs/index.html).
//! The JSON can come from either files or be located at HTTP endpoints.
//! 
//! Currently the module has methods that allow one to parse data from the `/state` or `/blocks`
//! endpoints of Hyperledger Sawtooth. 

extern crate serde_json;
extern crate reqwest;

use json_structs::json_blocks::{BlockData};
use json_structs::json_state::{StateData};

use std::fs;

/// Reads JSON data from the /blocks endpoint, but stored in a file.
/// Returns the JSON as a [BlockData](../json_structs/json_blocks/struct.BlockData.html) structure.
/// 
/// # Panics
/// This function will panic if the file cant be opened, read, or if it doesn't exist.
/// It will also panic if the structure of the JSON data is malformatted.
pub fn read_block_data_from_file(filepath: &str) -> BlockData {
    let file = fs::read_to_string(filepath)
        .expect("Unable to open file for reading block data: ");
 
    serde_json::from_str(file.as_str())
        .expect("Error in parsing block data file: ")
}

/// Reads JSON data from the /state endpoint, but stored in a file.
/// Returns the JSON as a [StateData](../json_structs/json_state/struct.StateData.html) structure.
/// 
/// # Panics
/// This function will panic if the file cant be opened, read, or if it doesn't exist.
/// It will also panic if the structure of the JSON data is malformatted.
pub fn read_state_data_from_file(filepath: &str) -> StateData {
    let file = fs::read_to_string(filepath)
        .expect("Unable to open file for reading state data: ");
 
    serde_json::from_str(file.as_str())
        .expect("Error in parsing state data file: ")
}

/// Reads JSON data from the /blocks endpoint using a `GET` request.
/// Returns the JSON as a [BlockData](../json_structs/json_blocks/struct.BlockData.html) structure.
/// 
/// # Panics
/// This function will panic if the request fails to be made, if the status code in the response 
/// is anything outisde of the 200 range, or if the JSON data is malformed.
pub fn read_block_data_from_endpoint(url: &str) -> BlockData {
    let mut response = reqwest::get(url).expect("Error in trying to make GET Request to server: ");
    if response.status().is_success() {
        let data: BlockData = response.json().expect("Error in parsing block JSON: ");
        data
    } else if response.status().is_server_error() || response.status().is_client_error(){
        panic!("Error code {} when trying to get /blocks endpoint: ", response.status().as_u16());
    } else {
        panic!("Unknown code {} when trying to get /blocks endpoint: ", response.status().as_u16());
    }
}

/// Reads JSON data from the `/state` endpoint using a `GET` request.
/// Returns the JSON as a [StateData](../json_structs/json_state/struct.StateData.html) structure.
///
/// # Panics
/// This function will panic if the request fails to be made, if the status code in the response 
/// is anything outisde of the 200 range, or if the JSON data is malformed.
pub fn read_state_data_from_endpoint(url: &str) -> StateData {
    let mut response = reqwest::get(url).expect("Error in trying to make GET Request to server: ");
    if response.status().is_success() {
        let data: StateData = response.json().expect("Error in parsing block JSON: ");
        data
    } else if response.status().is_server_error() || response.status().is_client_error(){
        panic!("Error code {} when trying to get /state endpoint: ", response.status().as_u16());
    } else {
        panic!("Bad code {} when trying to get /state endpoint: ", response.status().as_u16());
    }
}

#[cfg(test)]
mod test_read_from_file {
    use super::*;

    #[test]
    fn blockdata_valid_path_and_format() {
        let path = "example-blockchain/blocks.json";
        read_block_data_from_file(path);
    }

    #[test]
    #[should_panic(expected = "Unable to open file for reading block data:")]
    fn blockdata_invalid_path() {
        let path = "example-blockchain/does_not_exist.json";
        read_block_data_from_file(path);
    }

    #[test]
    #[should_panic(expected = "Error in parsing block data file:")]
    fn blockdata_valid_path_but_invalid_format() {
        let path = "example-blockchain/malformatted_block_data.json";
        read_block_data_from_file(path);
    }

    #[test]
    #[should_panic(expected = "Error in parsing block data file:")]
    fn blockdata_valid_path_but_wrong_file() {
        let path = "example-blockchain/state.json";
        read_block_data_from_file(path);
    }

    #[test]
    fn statedata_valid_path_and_format() {
        let path = "example-blockchain/state.json";
        read_state_data_from_file(path);
    }

    #[test]
    #[should_panic(expected = "Unable to open file for reading state data:")]
    fn statedata_invalid_path() {
        let path = "example-blockchain/does_not_exist.json";
        read_state_data_from_file(path);
    }

    #[test]
    #[should_panic(expected = "Error in parsing state data file:")]
    fn statedata_valid_path_but_invalid_format() {
        let path = "example-blockchain/malformatted_block_data.json";
        read_state_data_from_file(path);
    }

    #[test]
    #[should_panic(expected = "Error in parsing state data file:")]
    fn statedata_valid_path_but_wrong_file() {
        let path = "example-blockchain/blocks.json";
        read_state_data_from_file(path);
    }
}