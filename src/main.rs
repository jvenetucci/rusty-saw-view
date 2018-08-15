#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate colored;

pub mod json_blocks;
pub mod json_reader;
pub mod json_deserialize;

use json_blocks::{BlockData};
use json_reader::{read_block_data_from_file};

fn main() {
    let data: BlockData = read_block_data_from_file("example-blockchain/blocks.json");
    println!("Length: {}", data.get_num_blocks());
    data.display_full_data((false, false, String::from("cbor")));
}