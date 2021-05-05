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