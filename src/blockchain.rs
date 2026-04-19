use super::*;
#[derive(Debug)]
pub enum BlockValidationErr {
    MismatchedIndex,
    InvalidHash,
    AchronologicalTimestamp,
    MismatchedPreviousHash,
    InvalidGenesisBlockFormat,
}

pub struct Blockchain {
    pub blocks: Vec<Block>,
}
impl Blockchain {
    pub fn new () -> Self {
        Blockchain {
            blocks: vec![],
        }
    }

    pub fn update_with_block (&mut self, block: Block) -> Result<(), BlockValidationErr> {
        let i = self.blocks.len();

        if block.index != i as u32 {
            return Err(BlockValidationErr::MismatchedIndex);
        } else if !block::check_difficulty(&block.hash(), block.difficulty) {
            return Err(BlockValidationErr::InvalidHash);
        } else if i != 0 {
            // Not genesis block
            let prev_block = &self.blocks[i - 1];
            if block.timestamp <= prev_block.timestamp {
                return Err(BlockValidationErr::AchronologicalTimestamp);
            } else if block.prev_block_hash != prev_block.hash {
                return Err(BlockValidationErr::MismatchedPreviousHash);
            }
        } else {
            // Genesis block
            if block.prev_block_hash != vec![0; 32] {
                return Err(BlockValidationErr::InvalidGenesisBlockFormat);
            }
        };
        self.blocks.push(block);

        Ok(())
    }
    pub fn initialize_ledger(&mut self){
        let difficulty = 0x00ffffffffffffffffffffffffffffff;

        let mut genesis_block = Block::new(0, now(), vec![0; 32], vec![], difficulty);
        genesis_block.mine();
        println!("Mined genesis block {:?}", &genesis_block);    
        self.update_with_block(genesis_block).expect("Failed to add genesis block");
    }

}



