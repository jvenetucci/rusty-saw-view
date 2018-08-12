#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate serde_cbor;

extern crate base64;

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
    let json_data: BlockData = serde_json::from_str(file.unwrap().as_str()).unwrap();

    println!("There are {} blocks.", json_data.data.len());

    // Print out the blocks, the first 6 characters of their ID, and any transaction data available
    for block in json_data.data.iter() {
        println!("Block {} - {}", block.header.block_num, &block.header_signature[0..6]);
        if block.header.block_num != "0" { //Skip the genesis/settings block
            for batch in block.batches.iter() {
                for txn in batch.transactions.iter() {
                    // println!("\tData as Base64: {:x?}", txn.payload.as_str());

                    let bytes = base64::decode(txn.payload.as_str()).unwrap();
                    // println!("\tData as Byte Array: {:x?}", bytes);

                    let va: serde_cbor::Value = serde_cbor::from_slice(&bytes).unwrap();
                    let pq = va.as_object().unwrap();

                    for (key, val) in pq.iter() {
                        println!("\t\t{:?} : {:?}", key, val);
                    }
                }
            }
        }
        
    }
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
    println!("Length: {}", data.data.len());
    // read_user_from_file();
}