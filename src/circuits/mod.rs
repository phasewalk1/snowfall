pub mod common;
pub mod gadgets;
pub mod register;

use super::circuits::common::*;
use ark_crypto_primitives::crh::TwoToOneCRH;
use ark_crypto_primitives::merkle_tree::{Config, MerkleTree, Path};

#[derive(Clone)]
pub struct MerkleConfig;
impl Config for MerkleConfig {
    type LeafHash = LeafHash;
    type TwoToOneHash = TwoToOneHash;
}

pub type SnowfallMerkleTree = MerkleTree<MerkleConfig>;
pub type Root = <TwoToOneHash as TwoToOneCRH>::Output;
pub type SnowfallPath = Path<MerkleConfig>;
