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