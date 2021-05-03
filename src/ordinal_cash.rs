use std::collections::HashMap;

use crate::{merkle_tree::MerkleTreeWithHistory, U256};

pub trait Ordinal {
    fn process_deposit(&mut self, commitment: U256, insertedIndex: usize);
}

pub struct OrdinalCash<O: Ordinal> {
    pub mt: MerkleTreeWithHistory,
    commitments: HashMap<U256, bool>,
    o: O,
}

impl<O: Ordinal> OrdinalCash<O> {
    pub fn new(levels: u32, o: O) -> Self {
        let mt = MerkleTreeWithHistory::new(levels);
        let commitments = HashMap::new();

        Self { mt, commitments, o }
    }

    pub fn deposit(&mut self, commitment: U256) -> Option<()> {
        if self.commitments.contains_key(&commitment) {
            None
        } else {
            let inserted_index = self.mt.insert(commitment)?;
            self.commitments.insert(commitment, true);
            self.o.process_deposit(commitment, inserted_index);
            Some(())
        }
    }
}

pub struct SplOrdinal;

impl Ordinal for SplOrdinal {
    fn process_deposit(&mut self, commitment: U256, insertedIndex: usize) {
        //
        dbg!(insertedIndex);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deposit() {
        let levels = 2;
        let mut o = OrdinalCash::new(levels, SplOrdinal);
        assert!(o.deposit(U256::ONE).is_some());
        assert!(o.deposit(U256::ZERO).is_some());
    }

    #[test]
    fn test_deposit_should_fail_for_duplicate_commitment() {
        let levels = 2;
        let mut o = OrdinalCash::new(levels, SplOrdinal);
        assert!(o.deposit(U256::ONE).is_some());
        assert!(o.deposit(U256::ONE).is_none());
    }
}
