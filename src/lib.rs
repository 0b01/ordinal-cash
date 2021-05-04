use ark_bn254::FrParameters;

mod merkle_tree;
mod mimcsponge;
mod ordinal_cash;
mod pairing;
mod utils;

pub(crate) type U256 = ethnum::U256;
pub(crate) type Address = U256;
pub(crate) use crate::pairing::Proof;