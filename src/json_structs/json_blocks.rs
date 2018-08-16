// Joseph Venetucci

//! `json_blocks` is a collection of structures that represent the JSON structured 
//! data from the `/blocks` endpoint of Hyperledger Sawtooth. Also includes 
//! methods for retrieving useful information and displaying these structures.
//! 
//! Typically when you parse JSON data from the `/blocks` endpoint you'll store it in the root
//! [BlockData](struct.BlockData.html) structure. That structure contains methods for printing out
//! the contents of the blockchain. Use [display_full_data](struct.BlockData.html#method.display_full_data) when 
//! printing to the terminal, and [display_full_data_no_color](struct.BlockData.html#method.display_full_data_no_color)
//! if you want to pipe the output to a file.

use colored::*;
use json_deserialize::*;

/// A structure that represents the root data item found at the `/blocks` endpoint.
#[derive(Deserialize, Debug, Default)]
pub struct BlockData {
    data: Vec<Block>,
    head: String,
    link: String,
    paging: Paging
}

impl BlockData {

    /// Returns the number of blocks contained.
    pub fn get_num_blocks(&self) -> usize {
        self.data.len()
    }

    /// Display the individual blocks, their batches, and the transaction contained within them.
    /// Uses colored text so use this for terminal printing.
    /// 
    /// If instead you want to pipe this data to a file, see [display_full_data_no_color](struct.BlockData.html#method.display_full_data_no_color)
    /// since it prints with no coloring.
    /// 
    /// The function takes in a tuple (full_id, show_genesis, method) of settings:
    /// - `full_id` -> Setting this to false will only print the first 6 and last 4 characters of IDs and Pubkeys, 
    ///     otherwise the full string is shown.
    /// - `show_genesis` -> Setting this to false will omit the genesis block that sets blockchain setting data. It is
    ///     recommended that this stay false as its serialization will often be different from the rest of the data.
    /// - `method` -> What method to use when deserializing data. See
    ///     [supported methods](../../index.html#supported-deserialization-schemes) for a list of valid options.
    pub fn display_full_data(&self, (full_id, show_genesis, method): (bool, bool, String)) {
        
        // Figure out when to stop displaying arrows.
        let last_block_num = match show_genesis { true => "0", false => "1"};

        for block in self.data.iter() {
            // Only print out the first block if show_genesis is true
            if show_genesis || block.header.block_num != "0" {
                println!("{}{}{}", "|Block ".green().bold().on_black(), block.header.block_num.green().bold().on_black(), " ".on_black());
                if full_id {
                    println!("| ID: {}", block.header_signature.magenta());
                    println!("| Previous Block ID: {}", block.header.previous_block_id.magenta());
                    println!("| Signer Pub Key: {}", block.header.signer_public_key);
                } else {
                    // If not full_id, only print out the first 6, and last 4 characters
                    // Ok to do this since the following are always expected to be byte strings
                    println!("| ID: {}...{}", &block.header_signature[0..6].magenta(), &block.header_signature[(block.header_signature.len() - 4)..].magenta());
                    println!("| Previous Block ID: {}...{}", &block.header.previous_block_id[0..6].magenta(), &block.header.previous_block_id[(block.header.previous_block_id.len() - 4)..].magenta());
                    println!("| Signer Pub Key: {}...{}", &block.header.signer_public_key[0..6], &block.header.signer_public_key[(block.header.signer_public_key.len() - 4)..]);
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
                        println!("\t| ID: {}...{}", &batch.header_signature[0..6], &batch.header_signature[(batch.header_signature.len() - 4)..]);
                        println!("\t| Signer Pub Key: {}...{}", &batch.header.signer_public_key[0..6], &batch.header.signer_public_key[(batch.header.signer_public_key.len() - 4)..]);
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
                            println!("\t\t| ID: {}...{}", &txn.header_signature[0..6], &txn.header_signature[(txn.header_signature.len() - 4)..]);
                            println!("\t\t| Signer Pub Key: {}...{}", &txn.header.signer_public_key[0..6], &txn.header.signer_public_key[(txn.header.signer_public_key.len() - 4)..]);
                        }

                        // Deserialize the payload according to the passed in method
                        let payload_encoded = String::from(txn.payload.as_ref());

                        // If we are printing out the genesis block, don't deserialize the payload
                        if show_genesis && block.header.block_num == "0" {
                            println!("\t\t| Payload:\n{}", payload_encoded.blue())
                        } else {
                            match method.as_str() {
                                "cbor" => println!("\t\t| Payload:\n{}", parse_cbor(payload_encoded, 3).blue()),
                                _ => panic!("Unsupported deserialization method: {}", method)
                            }
                        }
                    }
                }
                // Display an arrow until we get to the last block
                if block.header.block_num != last_block_num {println!("{}", "\t\t| |\n\t\t| |\n\t\t\\ /\n\t\t V \n".green());}
            }
        }
    }

    /// Display the individual blocks, their batches, and the transaction contained within them.
    /// No terminal color, so use this for piping to a file.
    /// 
    /// If instead you want to print this to a terminal window, see [display_full_data_no_color](struct.BlockData.html#method.display_full_data)
    /// since it prints with coloring.
    /// 
    /// The function takes in a tuple (full_id, show_genesis, method) of settings:
    /// - `full_id` -> Setting this to false will only print the first 6 and last 4 characters of IDs and Pubkeys, 
    ///     otherwise the full string is shown.
    /// - `show_genesis` -> Setting this to false will omit the genesis block that sets blockchain setting data. It is
    ///     recommended that this stay false as its serialization will often be different from the rest of the data.
    /// - `method` -> What method to use when deserializing data. See
    ///     [supported methods](../../index.html#supported-deserialization-schemes) for a list of valid options.
    pub fn display_full_data_no_color(&self, (full_id, show_genesis, method): (bool, bool, String)) {
        
        // Figure out when to stop displaying arrows.
        let last_block_num = match show_genesis { true => "0", false => "1"};

        for block in self.data.iter() {
            // Only print out the first block if show_genesis is true
            if show_genesis || block.header.block_num != "0" {
                println!("{}{}{}", "|Block ", block.header.block_num, " ");
                if full_id {
                    println!("| ID: {}", block.header_signature);
                    println!("| Previous Block ID: {}", block.header.previous_block_id);
                    println!("| Signer Pub Key: {}", block.header.signer_public_key);
                } else {
                    // If not full_id, only print out the first 6, and last 4 characters
                    // Ok to do this since the following are always expected to be byte strings
                    println!("| ID: {}...{}", &block.header_signature[0..6], &block.header_signature[(block.header_signature.len() - 4)..]);
                    println!("| Previous Block ID: {}...{}", &block.header.previous_block_id[0..6], &block.header.previous_block_id[(block.header.previous_block_id.len() - 4)..]);
                    println!("| Signer Pub Key: {}...{}", &block.header.signer_public_key[0..6], &block.header.signer_public_key[(block.header.signer_public_key.len() - 4)..]);
                }

                match block.get_num_batches() {
                    1 => println!("| There is 1 batch in this block"),
                    count => println!("| There are {} batches in this block", count),
                }

                for (count, batch) in block.batches.iter().enumerate() {
                    println!("\t{}{}{}", "|Batch ", count.to_string(), " ");
                    if full_id {
                        println!("\t| ID: {}", batch.header_signature);
                        println!("\t| Signer Pub Key: {}", batch.header.signer_public_key);
                    } else {
                        println!("\t| ID: {}...{}", &batch.header_signature[0..6], &batch.header_signature[(batch.header_signature.len() - 4)..]);
                        println!("\t| Signer Pub Key: {}...{}", &batch.header.signer_public_key[0..6], &batch.header.signer_public_key[(batch.header.signer_public_key.len() - 4)..]);
                    }

                    match batch.get_num_txns() {
                    1 => println!("\t| There is 1 transaction in this batch"),
                    count => println!("\t| There are {} transactions in this batch", count),
                    }

                    for (count, txn) in batch.transactions.iter().enumerate() {
                        println!("\t\t{}{}{}", "|Transaction ", count.to_string(), " ");
                        if full_id {
                            println!("\t\t| ID: {}", txn.header_signature);
                            println!("\t\t| Signer Pub Key: {}", txn.header.signer_public_key);
                        } else {
                            println!("\t\t| ID: {}...{}", &txn.header_signature[0..6], &txn.header_signature[(txn.header_signature.len() - 4)..]);
                            println!("\t\t| Signer Pub Key: {}...{}", &txn.header.signer_public_key[0..6], &txn.header.signer_public_key[(txn.header.signer_public_key.len() - 4)..]);
                        }

                        // Deserialize the payload according to the passed in method
                        let payload_encoded = String::from(txn.payload.as_ref());

                        // If we are printing out the genesis block, don't deserialize the payload
                        if show_genesis && block.header.block_num == "0" {
                            println!("\t\t| Payload:\n{}", payload_encoded)
                        } else {
                            match method.as_str() {
                                "cbor" => println!("\t\t| Payload:\n{}", parse_cbor(payload_encoded, 3)),
                                _ => panic!("Unsupported deserialization method: {}", method)
                            }
                        } 
                    }
                }
                // Display an arrow until we get to the last block
                if block.header.block_num != last_block_num {println!("{}", "\t\t| |\n\t\t| |\n\t\t\\ /\n\t\t V \n");}
            }
        }
    }
}

/// A structure that represents a Block. Blocks contain metadata and a list of [Batches](struct.Batch.html). 
#[derive(Deserialize, Debug, Default)]
pub struct Block {
    batches: Vec<Batch>,
    header: BlockHeader,
    header_signature: String
}

impl Block {
    /// Returns the number of batches contained.
    pub fn get_num_batches(&self) -> usize {
        self.batches.len()
    }
}

/// A structure that represents the metadata of a [Block](struct.Block.html). 
#[derive(Deserialize, Debug, Default)]
pub struct BlockHeader {
    batch_ids: Vec<String>,
    block_num: String,
    consensus: String,
    previous_block_id: String,
    signer_public_key: String,
    state_root_hash: String
}

/// A structure that represents a Batch. Batches contain metadata and a list of [Transactions](struct.Transaction.html). 
#[derive(Deserialize, Debug, Default)]
pub struct Batch {
    header: BatchHeader,
    header_signature: String,
    trace: bool,
    transactions: Vec<Transaction>,
}

impl Batch {
    /// Returns the number of transactions contained.
    pub fn get_num_txns(&self) -> usize {
        self.transactions.len()
    }
}

/// A structure that represents the metadata of a [Batch](struct.Batch.html). 
#[derive(Deserialize, Debug, Default)]
pub struct BatchHeader {
    signer_public_key: String,
    transaction_ids: Vec<String>
}

/// A structure that represents a Transaction. Transactions contain metadata and a serialized payload encoded in Base64. 
#[derive(Deserialize, Debug, Default)]
pub struct Transaction {
    header: TransactionHeader,
    header_signature: String,
    payload: String
}

/// A structure that represents the metadata of a [Transaction](struct.Transaction.html). 
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

/// A structure that represents the paging element found at the `/blocks` endpoint. Only useful if the requester asked for paging.
#[derive(Deserialize, Debug, Default)]
pub struct Paging {
    limit: Option<String>,
    start: Option<String>,
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