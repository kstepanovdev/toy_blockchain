use super::*;

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn verify(&self) -> bool {
        for (i, block) in self.blocks.iter().enumerate() {
            if i as u32 != block.index {
                println!("Block index mismatch {} != {}", i, block.index);
                return false
            } else if !block::check_difficulty(&block.hash(), block.difficulty)  {
                println!("Block {} difficulty mismatch", i);
                return false
            } else if i != 0 {
                let prev_block = &self.blocks[i - 1];
                if block.timestamp <= prev_block.timestamp {
                    println!("Time should increase");
                    return false
                } else if block.previous_block_hash != prev_block.hash {
                    print!("Block linked hashes mismatch");
                    return false
                }
            } else {
                if block.previous_block_hash != [0; 32] {
                    print!("It isn't a Genesis block");
                }
            }
        }
        return true
    }
}
