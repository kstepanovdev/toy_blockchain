use toy_blockchainlib::*;

fn main() {
    let difficulty = 0x0000ffffffffffffffffffffffffffff;

    let mut genesis_block = Block::new(0, now(), vec![0; 32], vec![
        Transaction {
            inputs: vec![],
            outputs: 
                vec![
                    transaction::Output {
                        to_addr: "Alice".to_owned(),
                        value: 50
                    },
                    transaction::Output {
                        to_addr: "Bob".to_owned(),
                        value: 7
                    }
                ]
        }], difficulty);
    genesis_block.mine();
    let last_hash = genesis_block.hash();

    println!("Mined Genesis block {:?}", genesis_block);

    let mut blockchain = Blockchain::new();
    blockchain.update_with_block(genesis_block).expect("Failed to add Genesis block");

    let mut block = Block::new(1, now(), last_hash, vec![
        Transaction {
            inputs: vec![],
            outputs: 
                vec![
                    transaction::Output {
                        to_addr: "Chris".to_owned(),
                        value: 800
                    },
                ],
        },
        Transaction {
            inputs: vec![blockchain.blocks[0].transactions[0].outputs[0].clone()],
            outputs: 
                vec![
                    transaction::Output {
                        to_addr: "Alice".to_owned(),
                        value: 36
                    },
                    transaction::Output {
                        to_addr: "Bob".to_owned(),
                        value: 12
                    },

                ],
            }
        ], 
        difficulty);
    block.mine();

    println!("Mined Genesis block {:?}", block);

    blockchain.update_with_block(block).expect("Failed to add Another block");

}
