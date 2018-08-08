#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[derive(Deserialize, Debug)]
struct BlockData {
    data: Vec<Block>,
    head: String,
    link: String,
    paging: Paging
}

#[derive(Deserialize, Debug)]
struct Block {
    batches: Vec<Batch>,
    header: BlockHeader,
    header_signature: String
}

#[derive(Deserialize, Debug)]
struct BlockHeader {
    batch_ids: Vec<String>,
    block_num: String,
    consensus: String,
    previous_block_id: String,
    signer_public_key: String,
    state_root_hash: String
}

#[derive(Deserialize, Debug)]
struct Batch {
    header: BatchHeader,
    header_signature: String,
    trace: bool,
    transactions: Vec<Transaction>,
}

#[derive(Deserialize, Debug)]
struct BatchHeader {
    signer_public_key: String,
    transaction_ids: Vec<String>
}

#[derive(Deserialize, Debug)]
struct Transaction {
    header: TransactionHeader,
    header_signature: String,
    payload: String
}

#[derive(Deserialize, Debug)]
struct Paging {
    limit: serde_json::Value,
    start: Option<String>,
}

#[derive(Deserialize, Debug)]
struct TransactionHeader {
    batcher_public_key: String,
    dependencies: Vec<String>,
    family_name: String,
    family_version: String,
    inputs: Vec<String>,
    nonce: String,
    outputs: Vec<String>,
    payload_sha512: String,
    signer_public_key: String
}

#[derive(Deserialize, Debug)]
struct Person {
    name: String,
    age: u8
}

use std::fs;

fn read_user_from_file() -> () {
    let file = fs::read_to_string("example-blockchain/blocks.json");

    // Read JSON from file and store as a BlockData structure.
    let u: BlockData = serde_json::from_str(file.unwrap().as_str()).unwrap();

    println!("There are {} blocks.", u.data.len());

    // Print out the blocks, the first 6 characters of their ID, and any transaction data available
    for i in u.data.iter() {
        println!("Block {} - {}", i.header.block_num, &i.header_signature[0..6]);
        for j in i.batches.iter() {
            for k in j.transactions.iter() {
                println!("\tData: {}", k.payload);

            }
        }
    }
}

fn main() {
    read_user_from_file();
}