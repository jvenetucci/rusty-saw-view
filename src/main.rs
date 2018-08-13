#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate serde_cbor;

extern crate base64;

pub mod json_blocks;

use json_blocks::{BlockData};

use std::fs;

// fn read_user_from_file() -> () {
//     let file = fs::read_to_string("example-blockchain/blocks.json");

//     // Read JSON from file and store as a BlockData structure.
//     let json_data: json_blocks::BlockData = serde_json::from_str(file.unwrap().as_str()).unwrap();

//     println!("There are {} blocks.", json_data.data.len());

//     // Print out the blocks, the first 6 characters of their ID, and any transaction data available
//     for block in json_data.data.iter() {
//         println!("Block {} - {}", block.header.block_num, &block.header_signature[0..6]);
//         if block.header.block_num != "0" { //Skip the genesis/settings block
//             for batch in block.batches.iter() {
//                 for txn in batch.transactions.iter() {
//                     // println!("\tData as Base64: {:x?}", txn.payload.as_str());

//                     let bytes = base64::decode(txn.payload.as_str()).unwrap();
//                     // println!("\tData as Byte Array: {:x?}", bytes);

//                     let va: serde_cbor::Value = serde_cbor::from_slice(&bytes).unwrap();
//                     let pq = va.as_object().unwrap();

//                     for (key, val) in pq.iter() {
//                         println!("\t\t{:?} : {:?}", key, val);
//                     }
//                 }
//             }
//         }
        
//     }
// }

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
}

// Reads JSON data from the /blocks endpoint, but stored in a file.
// The /blocks endpoint contains data about the blocks in the chain.
// The function returns the JSON as a BlockData structure.
fn read_block_data_from_file(filepath: &str) -> BlockData {
    let file = fs::read_to_string(filepath)
        .expect("Unable to open file for reading block data: ");
 
    serde_json::from_str(file.as_str())
        .expect("Error in parsing block data file: ")
}

fn main() {
    let data: BlockData = read_block_data_from_file("example-blockchain/blocks.json");
    println!("Length: {}", data.get_num_blocks());
}