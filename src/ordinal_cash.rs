use crate::merkle_tree::MerkleTreeWithHistory;

pub struct OrdinalCash {
    pub mt: MerkleTreeWithHistory,
}

impl OrdinalCash {
    pub fn new(levels: u32) -> Self {
        let mt = MerkleTreeWithHistory::new(levels);

        Self {
            mt,
        }
    }
}