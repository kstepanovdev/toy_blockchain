use super::*;

#[derive(Debug)]
pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: Hash,
    pub previous_block_hash: Hash,
    pub nonce: u64,
    pub payload: String,
}

impl Block {
    pub fn new(index: u32, timestamp: u128, previous_block_hash: Hash, nonce: u64, payload: String) -> Block {
        Block {
            index,
            timestamp,
            hash: vec![0; 32],
            previous_block_hash,
            nonce,
            payload,
        }
    }
}

impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(&u32_bytes(&self.index));
        bytes.extend(&u128_bytes(&self.timestamp));
        bytes.extend(&self.previous_block_hash);
        bytes.extend(&u64_bytes(&self.nonce));
        bytes.extend(self.payload.as_bytes());

        bytes
    }

}