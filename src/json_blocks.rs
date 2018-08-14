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
                println!("{}{}{}", "|Block ".green().bold().on_black(), block.header.block_num.green().bold().on_black(), " ".on_black());
                if full_id {
                    println!("| ID: {}", block.header_signature.magenta());
                    println!("| Previous Block ID: {}", block.header.previous_block_id.magenta());
                    println!("| Signer Pub Key: {}", block.header.signer_public_key);
                } else {
                    println!("| ID: {}...{}", &block.header_signature[0..6].magenta(), &block.header_signature[0..4].magenta());
                    println!("| Previous Block ID: {}...{}", &block.header.previous_block_id[0..6].magenta(), &block.header.previous_block_id[0..4].magenta());
                    println!("| Signer Pub Key: {}...{}", &block.header.signer_public_key[0..6], &block.header.signer_public_key[0..4]);
                }

                match block.get_num_batches() {
                    1 => println!("| There is 1 batch in this block"),
                    count => println!("| There are {} batches in this block", count),
                }

                for (count, batch) in block.batches.iter().enumerate() {
                    println!("\t{}{}{}", "|Batch ".green().bold().on_black(), count.to_string().green().bold().on_black(), " ".on_black());
                    if full_id {
                        println!("\t| ID: {}", batch.header_signature);
                        println!("\t| Signer Pub Key: {}", batch.header.signer_public_key);
                    } else {
                        println!("\t| ID: {}...{}", &batch.header_signature[0..6], &batch.header_signature[0..4]);
                        println!("\t| Signer Pub Key: {}...{}", &batch.header.signer_public_key[0..6], &batch.header.signer_public_key[0..4]);
                    }

                    match batch.get_num_txns() {
                    1 => println!("\t| There is 1 transaction in this batch"),
                    count => println!("\t| There are {} transactions in this batch", count),
                    }

                    for (count, txn) in batch.transactions.iter().enumerate() {
                        println!("\t\t{}{}{}", "|Transaction ".green().bold().on_black(), count.to_string().green().bold().on_black(), " ".on_black());
                        if full_id {
                            println!("\t\t| ID: {}", txn.header_signature);
                            println!("\t\t| Signer Pub Key: {}", txn.header.signer_public_key);
                        } else {
                            println!("\t\t| ID: {}...{}", &txn.header_signature[0..6], &txn.header_signature[0..4]);
                            println!("\t\t| Signer Pub Key: {}...{}", &txn.header.signer_public_key[0..6], &txn.header.signer_public_key[0..4]);
                        }

                        println!("\t\t|\tData: {}", txn.payload.blue());

                    }
                }
                println!("{}", "\n\t\t| |\n\t\t| |\n\t\t\\ /\n\t\t V \n".green());
            }
        }
    }
}

#[cfg(test)]
mod test_blockdata_struct {
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

impl Block {
    pub fn get_num_batches(&self) -> usize {
        self.batches.len()
    }
}

#[cfg(test)]
mod test_block_struct {
    use super::*;

    #[test]
    fn empty_has_zero_batches() {
        let data = Block::default();
        assert_eq!(0, data.get_num_batches());
    }

    #[test]
    fn one_has_one_batch() {
        let mut data = Block::default();
        data.batches.push(Batch::default());
        assert_eq!(1, data.get_num_batches());
    }

    #[test]
    fn two_has_two_batches() {
        let mut data = Block::default();
        data.batches.push(Batch::default());
        data.batches.push(Batch::default());
        assert_eq!(2, data.get_num_batches());
    }
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

#[derive(Deserialize, Debug, Default)]
pub struct Batch {
    header: BatchHeader,
    header_signature: String,
    trace: bool,
    transactions: Vec<Transaction>,
}

impl Batch {
    pub fn get_num_txns(&self) -> usize {
        self.transactions.len()
    }
}

#[cfg(test)]
mod test_batch_struct {
    use super::*;

    #[test]
    fn empty_has_zero_txns() {
        let data = Batch::default();
        assert_eq!(0, data.get_num_txns());
    }

    #[test]
    fn one_has_one_txns() {
        let mut data = Batch::default();
        data.transactions.push(Transaction::default());
        assert_eq!(1, data.get_num_txns());
    }

    #[test]
    fn two_has_two_txns() {
        let mut data = Batch::default();
        data.transactions.push(Transaction::default());
        data.transactions.push(Transaction::default());
        assert_eq!(2, data.get_num_txns());
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct BatchHeader {
    signer_public_key: String,
    transaction_ids: Vec<String>
}

#[derive(Deserialize, Debug, Default)]
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

#[derive(Deserialize, Debug, Default)]
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