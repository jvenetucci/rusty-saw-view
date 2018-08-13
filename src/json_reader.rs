extern crate serde_json;

use json_blocks::{BlockData};

use std::fs;

// Reads JSON data from the /blocks endpoint, but stored in a file.
// The /blocks endpoint contains data about the blocks in the chain.
// The function returns the JSON as a BlockData structure.
pub fn read_block_data_from_file(filepath: &str) -> BlockData {
    let file = fs::read_to_string(filepath)
        .expect("Unable to open file for reading block data: ");
 
    serde_json::from_str(file.as_str())
        .expect("Error in parsing block data file: ")
}

#[cfg(test)]
mod test_read_from_file {
    use super::*;

    #[test]
    fn valid_path_and_format() {
        let path = "example-blockchain/blocks.json";
        read_block_data_from_file(path);
    }

    #[test]
    #[should_panic(expected = "Unable to open file for reading block data:")]
    fn invalid_path() {
        let path = "example-blockchain/does_not_exist.json";
        read_block_data_from_file(path);
    }

    #[test]
    #[should_panic(expected = "Error in parsing block data file:")]
    fn valid_path_but_invalid_format() {
        let path = "example-blockchain/malformatted_block_data.json";
        read_block_data_from_file(path);
    }

    #[test]
    #[should_panic(expected = "Error in parsing block data file:")]
    fn valid_path_but_wrong_file() {
        let path = "example-blockchain/state.json";
        read_block_data_from_file(path);
    }
}