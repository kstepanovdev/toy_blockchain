use toy_blockchainlib::*;

fn main() {
    let difficulty = 0x000fffffffffffffffffffffffffffff;

    let mut block = Block::new(0, 0, vec![0; 32], 0, "Genesis".to_string(), difficulty);

    block.mine();
    let mut last_hash = block.hash.clone();
    println!("{}", "Mine Genesis block");

    let mut blockchain = Blockchain {
        blocks: vec![block],
    };

    for i in 1..=10 {
        let mut block = Block::new(
            i,
            0,
            last_hash.clone(),
            0,
            "Another block".to_string(),
            difficulty,
        );

        block.mine();
        last_hash = block.hash();
        println!(
            "{}{}, it's hash is: {:?}",
            "Mined block #",
            i,
            hex::encode(&last_hash)
        );

        blockchain.blocks.push(block);
    }
}
