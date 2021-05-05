use ark_bn254::FrParameters;

mod merkle_tree;
mod mimcsponge;
mod ordinal_cash;
mod pairing;
mod utils;

pub type U256 = ethnum::U256;

pub type Address = U256;
pub(crate) use crate::pairing::Proof;

pub use ordinal_cash::{OrdinalCash, SplOrdinal};

#[macro_export]
macro_rules! bignum {
    ($c0: expr) => {
        {
        use ordinal_macros::to_sign_and_limbs;
        Box::new(U256::from_le_bytes(to_sign_and_limbs!($c0).1))
        }
    }
}