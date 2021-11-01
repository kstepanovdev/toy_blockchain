use toy_blockchainlib::*;

fn main() {
    let difficulty = 0x0000ffffffffffffffffffffffffffff;

    let mut block = Block::new(0, now(), vec![0; 32], 0, "Genesis".to_string(), difficulty);

    block.hash = block.hash();
    println!("{:?}", &block);

    block.mine();
    println!("{:?}", &block);
}
