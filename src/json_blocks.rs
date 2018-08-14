use colored::*; 

#[derive(Deserialize, Debug, Default)]
pub struct BlockData {
    data: Vec<Block>,
    head: String,
    link: String,
    paging: Paging
}

impl BlockData {
    pub fn get_num_blocks(&self) -> usize {
        self.data.len()
    }

    pub fn display_full_data(&self, (full_id, show_genesis): (bool, bool)) {
        // Print out the blocks
        for block in self.data.iter() {
            // Only print out the first block if full_id is true
            if show_genesis || block.header.block_num != "0" {
                println!("{}{}", "Block ".green().bold().on_black(), block.header.block_num.green().bold().on_black());
                if full_id {
                    println!("  ID: {}", block.header_signature.magenta());
                    println!("  Signer Pub Key: {}", block.header.signer_public_key);
                } else {
                    println!("  ID: {}...{}", &block.header_signature[0..6].magenta(), &block.header_signature[0..4].magenta());
                    println!("  Signer Pub Key: {}..{}", &block.header.signer_public_key[0..6], &block.header.signer_public_key[0..4]);
                }

            }
        //     if block.header.block_num != "0" { //Skip the genesis/settings block
        //         for batch in block.batches.iter() {
        //             for txn in batch.transactions.iter() {
        //                 // println!("\tData as Base64: {:x?}", txn.payload.as_str());

        //                 let bytes = base64::decode(txn.payload.as_str()).unwrap();
        //                 // println!("\tData as Byte Array: {:x?}", bytes);

        //                 let va: serde_cbor::Value = serde_cbor::from_slice(&bytes).unwrap();
        //                 let pq = va.as_object().unwrap();

        //                 for (key, val) in pq.iter() {
        //                     println!("\t\t{:?} : {:?}", key, val);
        //                 }
        //             }
        //         }
            }
        }
}

#[cfg(test)]
mod test_blockdata {
    use super::*;

    #[test]
    fn empty_has_zero_blocks() {
        let data = BlockData::default();
        assert_eq!(0, data.get_num_blocks());
    }

    #[test]
    fn one_has_one_block() {
        let mut data = BlockData::default();
        data.data.push(Block::default());
        assert_eq!(1, data.get_num_blocks());
    }

    #[test]
    fn two_has_two_blocks() {
        let mut data = BlockData::default();
        data.data.push(Block::default());
        data.data.push(Block::default());
        assert_eq!(2, data.get_num_blocks());
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct Block {
    batches: Vec<Batch>,
    header: BlockHeader,
    header_signature: String
}

#[derive(Deserialize, Debug, Default)]
pub struct BlockHeader {
    batch_ids: Vec<String>,
    block_num: String,
    consensus: String,
    previous_block_id: String,
    signer_public_key: String,
    state_root_hash: String
}

#[derive(Deserialize, Debug)]
pub struct Batch {
    header: BatchHeader,
    header_signature: String,
    trace: bool,
    transactions: Vec<Transaction>,
}

#[derive(Deserialize, Debug)]
pub struct BatchHeader {
    signer_public_key: String,
    transaction_ids: Vec<String>
}

#[derive(Deserialize, Debug)]
pub struct Transaction {
    header: TransactionHeader,
    header_signature: String,
    payload: String
}

#[derive(Deserialize, Debug, Default)]
pub struct Paging {
    limit: Option<String>,
    start: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct TransactionHeader {
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