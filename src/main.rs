#[macro_use]
extern crate serde_derive;

extern crate serde;

extern crate colored;

use colored::*;

pub mod json_blocks;
pub mod json_reader;
pub mod json_deserialize;

use json_blocks::{BlockData};
use json_reader::{read_block_data_from_file};

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

fn main() {
    let data: BlockData = read_block_data_from_file("example-blockchain/blocks.json");
    println!("Length: {}", data.get_num_blocks());
    data.display_full_data((false, false, String::from("cbor")));
}