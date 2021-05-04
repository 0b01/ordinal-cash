use std::collections::HashMap;

use crate::{merkle_tree::MerkleTreeWithHistory, U256};

pub trait Ordinal {
    fn process_deposit(&mut self, commitment: U256, inserted_index: usize);
    fn process_withdraw(&mut self, recipient: crate::Address, relayer: crate::Address, fee: U256, refund: U256);
}

pub struct OrdinalCash<O: Ordinal> {
    pub mt: MerkleTreeWithHistory,
    commitments: HashMap<U256, bool>,
    nullifier_hashes: HashMap<U256, bool>,
    o: O,
}

impl<O: Ordinal> OrdinalCash<O> {
    pub fn new(levels: u32, o: O) -> Self {
        let mt = MerkleTreeWithHistory::new(levels);
        let commitments = HashMap::new();
        let nullifier_hashes = HashMap::new();

        Self { mt, commitments, nullifier_hashes, o }
    }

    pub fn deposit(&mut self, commitment: U256) -> Option<()> {
        if self.commitments.contains_key(&commitment) {
            // The commitment already exists
            None
        } else {
            let inserted_index = self.mt.insert(commitment)?;
            self.commitments.insert(commitment, true);
            self.o.process_deposit(commitment, inserted_index);
            Some(())
        }
    }

    pub fn withdraw(&mut self, proof: crate::Proof, root: U256, nh: U256, recipient: crate::Address, relayer: crate::Address, fee: U256, refund: U256) -> Option<()> {
        // require(_fee <= denomination, "Fee exceeds transfer value");
        if self.nullifier_hashes.contains_key(&nh) && !self.nullifier_hashes[&nh] {
            // The note has been already spent
            None
        } else if !self.mt.is_known_root(root) {
            // Cannot find your merkle root
            None
        } else if !crate::pairing::verify_proof(proof, &[root, nh, recipient, relayer, fee, refund]) {
            // invalid withdraw proof
            None
        } else {
            self.nullifier_hashes[&nh] = true;
            self.o.process_withdraw(recipient, relayer, fee, refund);
            // emit Withdrawal(_recipient, _nullifierHash, _relayer, _fee);

            Some(())
        }
    }
}

pub struct SplOrdinal;

impl Ordinal for SplOrdinal {
    fn process_deposit(&mut self, commitment: U256, inserted_index: usize) {
        //
        dbg!(inserted_index);
    }
    fn process_withdraw(&mut self, recipient: crate::Address, relayer: crate::Address, fee: U256, refund: U256) {
        //
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
