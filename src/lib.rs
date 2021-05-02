mod mimcsponge;
mod utils;

use ethnum::U256;
use mimcsponge::mimcsponge;
use utils::addmod;

const ROOT_HISTORY_SIZE: usize = 100;

pub struct MerkleTreeWithHistory {
    pub levels: u32,
    pub filledSubtrees: Vec<u32>,
    pub zeros: Vec<u32>,
    pub currentRootIndex: usize,
    pub nextIndex: usize,
    pub roots: [u32; ROOT_HISTORY_SIZE],
    pub FIELD_SIZE: U256,
    pub ZERO_VALUE: U256,
}

impl MerkleTreeWithHistory {

    pub fn new(treeLevels: u32) -> Self {
        assert!(treeLevels > 0, "_treeLevels should be greater than zero");
        assert!(treeLevels < 32, "_treeLevels should be less than 32");

        let FIELD_SIZE: U256 = U256::from_str_radix("21888242871839275222246405745257275088548364400416034343698204186575808495617", 10).unwrap();
        let ZERO_VALUE: U256 = U256::from_str_radix("21663839004416932945382355908790599225266501822907911457504978515578255421292", 10).unwrap(); // = keccak256("tornado") % FIELD_SIZE

        let levels = treeLevels;
        let filledSubtrees = Vec::new();
        let zeros = Vec::new();
        let roots = [0; ROOT_HISTORY_SIZE];
        let mut this = Self {
            levels,
            filledSubtrees,
            zeros,
            currentRootIndex: 0,
            nextIndex: 0,
            roots,
            FIELD_SIZE,
            ZERO_VALUE,
        };

        let mut currentZero = ZERO_VALUE.as_u32();
        this.zeros.push(currentZero);
        this.filledSubtrees.push(currentZero);

        for i in 1..levels {
            currentZero = this.hashLeftRight(currentZero, currentZero);
            this.zeros.push(currentZero);
            this.filledSubtrees.push(currentZero);
        }

        this.roots[0] = this.hashLeftRight(currentZero, currentZero);
        this
    }

    pub fn hashLeftRight(&self, left: u32, right: u32) -> u32 {
        assert!(U256::from(left) < self.FIELD_SIZE, "_left should be inside the field");
        assert!(U256::from(right) < self.FIELD_SIZE, "_right should be inside the field");
        let R = U256::from(left);
        let C = U256::new(0);
        let (mut R, C) = mimcsponge(R, C, self.FIELD_SIZE);
        R = addmod(R, U256::from(right), self.FIELD_SIZE);
        let (R, C) = mimcsponge(R, C, self.FIELD_SIZE);
        return R.as_u32();
    }

    pub fn insert(&mut self, leaf: u32) -> u32 {
        let mut currentIndex = self.nextIndex;
        assert!(currentIndex != 2_usize.saturating_pow(self.levels), "Merkle tree is full. No more leafs can be added");
        self.nextIndex += 1;
        let mut currentLevelHash = leaf;
        let mut left;
        let mut right;

        for i in 0..(self.levels as usize) {
            if currentIndex % 2 == 0 {
                left = currentLevelHash;
                right = self.zeros[i];

                self.filledSubtrees[i] = currentLevelHash;
            } else {
                left = self.filledSubtrees[i];
                right = currentLevelHash;
            }

            currentLevelHash = self.hashLeftRight(left, right);

            currentIndex /= 2;
        }

        self.currentRootIndex = (self.currentRootIndex + 1) % ROOT_HISTORY_SIZE;
        self.roots[self.currentRootIndex] = currentLevelHash;
        return self.nextIndex as u32 - 1;
    }

    pub fn isKnownRoot(&self, root: u32) -> bool {
        if root == 0 {
            return false;
        }
        let mut i = self.currentRootIndex;
        loop {
            if root == self.roots[i] {
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

    pub fn getLastRoot(&self) -> u32 {
        self.roots[self.currentRootIndex]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merkletree_new() {
        let mt = MerkleTreeWithHistory::new(16);
        assert_eq!(mt.filledSubtrees[0], mt.ZERO_VALUE.as_u32());
        assert_eq!(mt.zeros[0], mt.ZERO_VALUE.as_u32());
    }

    #[test]
    fn test_merkletree_insert_single() {
        let mut mt = MerkleTreeWithHistory::new(16);
        mt.insert(5);
        let expected = U256::from_str_radix("21305827034995891902714687670641862055126514524916463201449278400604999416145", 10).unwrap();
        let root = mt.getLastRoot();
        dbg!(root);
        assert_eq!(root, expected.as_u32());
    }

    // #[test]
    // fn test_merkletree_insert() {
    //     let mut mt = MerkleTreeWithHistory::new(16);
    //     let expected = vec![
    //         U256::from_str_radix("4768177824216143055519640784977157311673229774898220393681621414098673579769", 10).unwrap(),
    //         U256::from_str_radix("19250388085485041587583085418097442146754159803884683357829010263306957939672", 10).unwrap(),
    //         U256::from_str_radix("9689626096216832334627417845654134122170488617637736288368858062365673557464", 10).unwrap(),
    //         U256::from_str_radix("21700338360469497276660148715400634886688877896825241122038049374177304387023", 10).unwrap(),
    //         U256::from_str_radix("10894114695301286167584690769534390299722097888629758484993940021677274863769", 10).unwrap(),
    //         U256::from_str_radix("4351731605390097568003558296954911732518522051938980049615185907037960668614", 10).unwrap(),
    //         U256::from_str_radix("10346662249939065006333306573805808501397464568719002016595968522109223325250", 10).unwrap(),
    //         U256::from_str_radix("19032707132583078368821576258242660551222155546143830081600126918958204483243", 10).unwrap(),
    //         U256::from_str_radix("13042599070212323078869215498376988042181613713259787038594716518883272253920", 10).unwrap(),
    //         U256::from_str_radix("15200063891796499502721825879098395282261979630510984497744981505913469850275", 10).unwrap(),
    //     ];

    //     for i in 1_usize..11 {
    //         assert_eq!(mt.getLastRoot(), expected[i-1].as_u32(), "{}", i-1);
    //         mt.insert(i as u32);
    //     }
    // }
}