use crate::mimcsponge::MimcSponge;
use crate::utils::addmod;
use crate::U256;
#[macro_use]
use crate::bignum;

const ROOT_HISTORY_SIZE: usize = 100;

#[derive(Default)]
pub struct MerkleTreeWithHistory {
    pub levels: u32,
    pub filledSubtrees: Box<Vec<Box<U256>>>,
    pub zeros: Box<Vec<Box<U256>>>,
    pub currentRootIndex: usize,
    pub nextIndex: usize,
    pub roots: Box<Vec<Box<U256>>>,
    pub FIELD_SIZE: Box<U256>,
    pub ZERO_VALUE: Box<U256>,
    pub sponge: Box<MimcSponge>,
}

impl MerkleTreeWithHistory {
    pub fn new(levels: u32) -> Self {
        let mut this: Self = Default::default();
        assert!(levels > 0, "_treeLevels should be greater than zero");
        assert!(levels < 32, "_treeLevels should be less than 32");

        let FIELD_SIZE = bignum!("21888242871839275222246405745257275088548364400416034343698204186575808495617");
        let ZERO_VALUE = bignum!("21663839004416932945382355908790599225266501822907911457504978515578255421292");

        this.levels = levels;
        this.roots = Box::new(vec![Box::new(U256::ZERO); ROOT_HISTORY_SIZE]);
        this.FIELD_SIZE = FIELD_SIZE.clone();
        this.ZERO_VALUE = ZERO_VALUE.clone();

        let mut currentZero = ZERO_VALUE.clone();
        this.zeros.push(currentZero.clone());
        this.filledSubtrees.push(currentZero.clone());

        for i in 1..levels {
            currentZero = this.hashLeftRight(&currentZero, &currentZero);
            this.zeros.push(currentZero.clone());
            this.filledSubtrees.push(currentZero.clone());
        }

        this.roots[0] = this.hashLeftRight(&currentZero, &currentZero);
        this
    }

    pub fn hashLeftRight(&self, left: &U256, right: &U256) -> Box<U256> {
        // assert!( left < &self.FIELD_SIZE, "_left should be inside the field");
        // assert!( right < self.FIELD_SIZE, "_right should be inside the field");
        let R = left;
        let C = U256::new(0);
        let (mut R, C) = self.sponge.mimcsponge(&R, &C, &self.FIELD_SIZE);
        R = addmod(&R, &right, &self.FIELD_SIZE);
        let (R, C) = self.sponge.mimcsponge(&R, &C, &self.FIELD_SIZE);
        Box::new(R)
    }

    pub fn insert(&mut self, leaf: &U256) -> Option<usize> {
        let mut currentIndex = self.nextIndex;
        if currentIndex == 2_usize.saturating_pow(self.levels) {
            //"Merkle tree is full. No more leafs can be added");
            return None;
        }

        self.nextIndex += 1;
        let mut currentLevelHash: Box<U256> = Box::new(*leaf);
        let mut left: &U256;
        let mut right: &U256;

        for i in 0..(self.levels as usize) {
            if currentIndex % 2 == 0 {
                left = &currentLevelHash;
                right = &self.zeros[i];

                self.filledSubtrees[i] = currentLevelHash.clone();
            } else {
                left = &self.filledSubtrees[i];
                right = &currentLevelHash;
            }

            currentLevelHash = self.hashLeftRight(left, right);

            currentIndex /= 2;
        }

        self.currentRootIndex = (self.currentRootIndex + 1) % ROOT_HISTORY_SIZE;
        self.roots[self.currentRootIndex] = Box::new(*currentLevelHash);
        Some(self.nextIndex as usize - 1)
    }

    pub fn is_known_root(&self, root: &U256) -> bool {
        if root == &U256::ZERO {
            return false;
        }
        let mut i = self.currentRootIndex;
        loop {
            if *root == *self.roots[i] {
                return true;
            }
            if i == 0 {
                i = ROOT_HISTORY_SIZE;
            }

            i -= 1;

            if i == self.currentRootIndex {
                break;
            }
        }

        false
    }

    pub fn getLastRoot(&self) -> Box<U256> {
        self.roots[self.currentRootIndex].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merkletree_new() {
        let mt = MerkleTreeWithHistory::new(16);
        assert_eq!(mt.filledSubtrees[0], mt.ZERO_VALUE);
        assert_eq!(mt.zeros[0], mt.ZERO_VALUE);
    }

    #[test]
    fn test_merkletree_insert_single() {
        let mut mt = MerkleTreeWithHistory::new(2);
        mt.insert(&U256::new(5));
        let expected = bignum!("21305827034995891902714687670641862055126514524916463201449278400604999416145");
        let root = mt.getLastRoot();
        assert_eq!(root, expected);
    }

    #[test]
    fn test_merkletree_insert_single_3() {
        let mut mt = MerkleTreeWithHistory::new(3);
        mt.insert(&U256::new(1));
        let expected = bignum!("14817887234532324632578486942317778767513333548116388705259454362287888156301");
        let root = mt.getLastRoot();
        assert_eq!(root, expected);
    }

    #[test]
    fn test_merkletree_insert_single_16() {
        let mut mt = MerkleTreeWithHistory::new(16);
        mt.insert(&U256::new(5));
        let expected = bignum!("20078220768011993253497856250024317483006104588209594787144509816521675548945");
        assert_eq!(mt.currentRootIndex, 1);
        let root = mt.getLastRoot();
        assert_eq!(root, expected);
    }

    #[test]
    fn test_merkletree_insert() {
        let mut mt = MerkleTreeWithHistory::new(16);
        let expected = vec![
            bignum!("3431256714363396804770991575090970055302175921802683225882378599453141462503"),
            bignum!("7575821202546991722047889195143698024641067539407824397010939985717182566799"),
            bignum!("7102419650151881575380791103194015368648640006236895399277475380346088306449"),
            bignum!("3663265918960820756765744378616083555095944410653161772251208095179127101510"),
            bignum!("15302658532613586889202868102641369060511299011842796454718345900410135644534"),
            bignum!("19867311980617909474730049456052719869948526667934900087741729669853083711560"),
            bignum!("6061878619835624285838818217971195365504071979555702464817484176105688178577"),
            bignum!("2521963888311190328687829229664642120391801081246544527123137783093792814465"),
            bignum!("10214875608306830392931189580024717263641319338206990452441323784791611321245"),
            bignum!("7692234562883530752899755807890957688721742766928110244142163893445927985263"),
        ];

        for i in 1_usize..11 {
            mt.insert(&U256::new(i as u128));
            assert_eq!(mt.currentRootIndex, i);
            assert_eq!(mt.getLastRoot(), expected[i - 1], "{}", i);
        }
    }

    #[test]
    fn test_tree_full() {
        let levels = 6;
        let mut mt = MerkleTreeWithHistory::new(6);

        for i in 0..(2_u128.pow(levels)) {
            assert!(mt.insert(&U256::new(i + 42)).is_some());
        }

        assert!(mt.insert(&U256::new(1337)).is_none());
    }

    #[test]
    fn test_is_known_root() {
        let mut mt = MerkleTreeWithHistory::new(6);

        for i in 1..5 {
            mt.insert(&U256::new(i));
            assert!(mt.is_known_root(&mt.roots[0]));
        }

        assert!(!mt.is_known_root(&U256::new(0)));
    }

    #[test]
    fn test_insert_root() {
        let mut mt = MerkleTreeWithHistory::new(16);
        mt.insert(&*bignum!("8144601074668623426925770169834644636770764159380454737463139103752848208415"));
        let expected_root = bignum!("18759831220824932236585314001088159476096807910838182935046606337929711439019");
        assert_eq!(expected_root, mt.roots[1]);
    }

    #[test]
    fn test_insert_root_2() {
        let mut mt = MerkleTreeWithHistory::new(20);
        mt.insert(&*bignum!("8144601074668623426925770169834644636770764159380454737463139103752848208415"));
        let expected_root = bignum!("18141211044530898481780712096785380507009040886197825359491225784587697908689");
        assert_eq!(expected_root, mt.roots[1]);
    }
}
