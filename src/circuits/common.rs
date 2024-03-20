use crate::circuits::gadgets::{LeafHashGadget, TwoToOneHashGadget};
use ark_crypto_primitives::{
    commitment::pedersen::Commitment,
    crh::{
        constraints::{CRHGadget, TwoToOneCRHGadget},
        injective_map::{PedersenCRHCompressor, TECompressor},
        pedersen,
    },
    merkle_tree::constraints::PathVar,
};
use ark_ed_on_bls12_381::{constraints::EdwardsVar, EdwardsProjective, Fq, Fr};
use ark_r1cs_std::fields::fp::FpVar;

/// A public key
pub type PublicKey = Fq;

/// The r1cs equivalent of a public key
pub type PublicKeyVar = FpVar<Fq>;

/// A scalar (should be Fr, but I don't wanna implement a custom ConstraintSytem rn -- maybe break
/// into two circuits?)
pub type Gamma = Fq;

/// The r1cs equivalent of Gamma
pub type GammaVar = FpVar<Fq>;

/// A commitment
pub type PCommitment = Commitment<EdwardsProjective, windows::PedersenCommWindow>;

/// The r1cs equivalent of a commitment
pub type CommitmentVar = EdwardsVar;

/// The r1cs equivalent of the MerkleTree root
pub type RootVar = <TwoToOneHashGadget as TwoToOneCRHGadget<TwoToOneHash, ConstraintF>>::OutputVar;

/// The r1cs equivalent of the MerkleTree path
pub type SnowfallPathVar = PathVar<
    crate::circuits::register::MerkleConfig,
    LeafHashGadget,
    TwoToOneHashGadget,
    ConstraintF,
>;

/// 2-to-1 hash function for the Merkle tree
pub type TwoToOneHash =
    PedersenCRHCompressor<EdwardsProjective, TECompressor, windows::TwoToOneWindow>;
/// Leaf hash function for the Merkle tree
pub type LeafHash = PedersenCRHCompressor<EdwardsProjective, TECompressor, windows::LeafWindow>;

/// The scalar field used in the circuit
pub type ConstraintF = ark_ed_on_bls12_381::Fq;

/// The r1cs equivalent of the parameters for the 2-to-1 hash function
pub type LeafHashParamsVar = <LeafHashGadget as CRHGadget<LeafHash, ConstraintF>>::ParametersVar;

/// The r1cs equivalent of the parameters for the leaf hash function
pub type TwoToOneHashParamsVar =
    <TwoToOneHashGadget as TwoToOneCRHGadget<TwoToOneHash, ConstraintF>>::ParametersVar;

/// Windows for hashers
pub mod windows {
    use super::pedersen;

    /// Window for Pedersen commitments
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct PedersenCommWindow;

    impl pedersen::Window for PedersenCommWindow {
        const WINDOW_SIZE: usize = 4;
        const NUM_WINDOWS: usize = 128;
    }

    /// Window for 2-to-1 hash functions in the Merkle tree
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct TwoToOneWindow;

    impl pedersen::Window for TwoToOneWindow {
        const WINDOW_SIZE: usize = 4;
        const NUM_WINDOWS: usize = 128;
    }

    /// Window for leaf hash functions in the Merkle tree
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct LeafWindow;

    impl pedersen::Window for LeafWindow {
        const WINDOW_SIZE: usize = 4;
        const NUM_WINDOWS: usize = 144;
    }
}
