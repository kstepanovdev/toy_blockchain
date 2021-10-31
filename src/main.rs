use toy_blockchainlib::*;

fn main() {
    let mut block = Block::new(0, now(), vec![0; 32], 0, "Genesis".to_string());
    println!("{:?}", block);

    let h = block.hash();
    println!("{:?}", &h);

    block.hash = h;
    println!("{:?}", block);

}
