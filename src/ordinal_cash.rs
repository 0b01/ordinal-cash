use std::collections::HashMap;
use ark_ff::{field_new};
use ark_bn254::{Fr, Fq, Fq2, G1Affine, G2Affine};
use crate::{merkle_tree::MerkleTreeWithHistory, U256, Proof};

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

    pub fn withdraw(&mut self, proof: Proof, root: U256, nh: U256, recipient: crate::Address, relayer: crate::Address, fee: U256, refund: U256) -> Result<(), &'static str> {
        // require(_fee <= denomination, "Fee exceeds transfer value");
        if self.nullifier_hashes.contains_key(&nh) && !self.nullifier_hashes[&nh] {
            Err("The note has been already spent")
        } else if !self.mt.is_known_root(root) {
            Err("Cannot find your merkle root")
        } else if !crate::pairing::verify_proof(proof, &[
                to_fr(&root)?,
                to_fr(&nh)?,
                to_fr(&recipient)?,
                to_fr(&relayer)?,
                to_fr(&fee)?,
                to_fr(&refund)?
            ]) {
            Err("Invalid withdraw proof")
        } else {
            self.nullifier_hashes.insert(nh, true);
            self.o.process_withdraw(recipient, relayer, fee, refund);
            // emit Withdrawal(_recipient, _nullifierHash, _relayer, _fee);
            Ok(())
        }
    }
}

pub fn to_fr(e: &U256) -> Result<ark_bn254::Fr, &'static str> {
    use ark_ff::FromBytes;
    let bytes = e.to_le_bytes();
    Fr::read(std::io::Cursor::new(bytes)).map_err(|_| "cannot read bytes")
}

pub struct SplOrdinal;

impl Ordinal for SplOrdinal {
    fn process_deposit(&mut self, commitment: U256, inserted_index: usize) {
        //
        dbg!(inserted_index);
    }
    fn process_withdraw(&mut self, recipient: crate::Address, relayer: crate::Address, fee: U256, refund: U256) {
        //
        dbg!(recipient);
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

    #[test]
    fn test_u256_to_fr() {
        use ark_ff::PrimeField;
        assert_eq!(
            field_new!(Fr, "18141211044530898481780712096785380507009040886197825359491225784587697908689"),
            to_fr(&U256::from_str_radix("18141211044530898481780712096785380507009040886197825359491225784587697908689", 10).unwrap()).unwrap(),
        );
    }

    #[test]
    fn test_deposit_then_withdraw() {
        use ark_bn254::{Fq2, Fq, Fr};
        let levels = 20;
        let mut o = OrdinalCash::new(levels, SplOrdinal);
        // 8144601074668623426925770169834644636770764159380454737463139103752848208415
        let commitment = U256::from_str_radix("8144601074668623426925770169834644636770764159380454737463139103752848208415", 10).unwrap();
        assert!(o.deposit(commitment).is_some());

        /*
        let proofData = {
            pi_a: [
                '6160569254972695033541751837268483084366769085559190351203251273104161418400',
                '20752637131085270521468616099378777282140643373828587571739440568509124276056',
                '1'
            ],
            pi_b: [
                [
                '13491861502261516983101645835022566763609289651768567310911403618761646949192',
                '3945102381244376369012265840723718630151501000113196168586847393173187043791'
                ],
                [
                '2085938618828745684062388752928952586729586420152554769557897926110027529638',
                '6905794850663567498189336560369926542004763485239983052761138480096642452652'
                ],
                [ '1', '0' ]
            ],
            pi_c: [
                '13305831737468453023847968374851929911131764663101012129143182418423488429101',
                '8142609777444797171303864593507761446494142436560247433779193992616167019550',
                '1'
            ],
            publicSignals: [
                '18141211044530898481780712096785380507009040886197825359491225784587697908689',
                '17369391381428457005685637744737812745147294406289952788882032335952086150537',
                '617288482572789990873151114501867268774234674064',
                '827641930419614124039720421795580660909102123457',
                '50000000000000000',
                '0'
            ]
        };
        */

        let a = G1Affine::new(
            field_new!(
                Fq,
                "6160569254972695033541751837268483084366769085559190351203251273104161418400"
            ),
            field_new!(
                Fq,
                "20752637131085270521468616099378777282140643373828587571739440568509124276056"
            ),
            false,
        );

        let b = G2Affine::new(
            field_new!(
                Fq2,
                field_new!(
                    Fq,
                    "13491861502261516983101645835022566763609289651768567310911403618761646949192"
                ),
                field_new!(
                    Fq,
                    "3945102381244376369012265840723718630151501000113196168586847393173187043791"
                ),
            ),
            field_new!(
                Fq2,
                field_new!(
                    Fq,
                    "2085938618828745684062388752928952586729586420152554769557897926110027529638"
                ),
                field_new!(
                    Fq,
                    "6905794850663567498189336560369926542004763485239983052761138480096642452652"
                ),
            ),
            false,
        );

        let c = G1Affine::new(
            field_new!(
                Fq,
                "13305831737468453023847968374851929911131764663101012129143182418423488429101"
            ),
            field_new!(
                Fq,
                "8142609777444797171303864593507761446494142436560247433779193992616167019550"
            ),
            false,
        );

        let ret = o.withdraw(Proof{a,b,c},
            U256::from_str_radix("18141211044530898481780712096785380507009040886197825359491225784587697908689", 10).unwrap(),
            U256::from_str_radix("17369391381428457005685637744737812745147294406289952788882032335952086150537", 10).unwrap(),
            U256::from_str_radix("617288482572789990873151114501867268774234674064", 10).unwrap(),
            U256::from_str_radix("827641930419614124039720421795580660909102123457", 10).unwrap(),
            U256::from_str_radix("50000000000000000", 10).unwrap(),
            U256::from_str_radix("0", 10).unwrap(),
        );

        assert!(ret.is_ok());
    }
}
