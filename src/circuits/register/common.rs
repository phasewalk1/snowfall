use ark_crypto_primitives::{
    crh::{constraints::{CRHGadget, TwoToOneCRHGadget},
        injective_map::constraints::{
            PedersenCRHCompressorGadget, TECompressorGadget,
        },
        injective_map::{PedersenCRHCompressor, TECompressor},
        pedersen,
    },
    commitment::pedersen::Commitment,
    merkle_tree::constraints::PathVar,
};
use ark_ed_on_bls12_381::{
    constraints::EdwardsVar,
    EdwardsProjective,
    Fq, Fr
};
use ark_r1cs_std::fields::fp::FpVar;
use ark_relations::r1cs::{ConstraintSystem, ConstraintSystemRef};

/// A public key
pub type PublicKey = Fq;

/// A scalar (should be Fr, but we use Fq for simplicity so we can use the same circuit for both)
pub type Gamma = Fq;

/// The r1cs equivalent of Gamma
pub type GammaVar = FpVar<Fq>;

/// A commitment
pub type PCommitment = Commitment<EdwardsProjective, super::common::windows::PedersenCommWindow>;

/// The r1cs equivalent of a public PublicKey
pub type PublicKeyVar = FpVar<Fq>;

/// The r1cs equivalent of the MerkleTree root
pub type RootVar = <TwoToOneHashGadget as TwoToOneCRHGadget<TwoToOneHash, ConstraintF>>::OutputVar;

/// The r1cs equivalent of the MerkleTree path
pub type SnowfallPathVar = 
    PathVar<super::MerkleConfig, LeafHashGadget, TwoToOneHashGadget, ConstraintF>;

/// The r1cs equivalent of a commitment
pub type CommitmentVar = EdwardsVar;

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


/// 2-to-1 hash function for the Merkle tree
pub type TwoToOneHash = PedersenCRHCompressor<EdwardsProjective, TECompressor, windows::TwoToOneWindow>;
/// Leaf hash function for the Merkle tree
pub type LeafHash = PedersenCRHCompressor<EdwardsProjective, TECompressor, windows::LeafWindow>;

/// A gadget for the 2-to-1 hash function
pub type TwoToOneHashGadget = PedersenCRHCompressorGadget<
    EdwardsProjective,
    TECompressor,
    windows::TwoToOneWindow,
    EdwardsVar,
    TECompressorGadget,
>;

/// A gadget for the leaf hash function
pub type LeafHashGadget = PedersenCRHCompressorGadget<
    EdwardsProjective,
    TECompressor,
    windows::LeafWindow,
    EdwardsVar,
    TECompressorGadget,
>;

/// The scalar field used in the circuit
pub type ConstraintF = ark_ed_on_bls12_381::Fq;

/// The r1cs equivalent of the parameters for the 2-to-1 hash function
pub type LeafHashParamsVar = <LeafHashGadget as CRHGadget<LeafHash, ConstraintF>>::ParametersVar;

/// The r1cs equivalent of the parameters for the leaf hash function
pub type TwoToOneHashParamsVar =
    <TwoToOneHashGadget as TwoToOneCRHGadget<TwoToOneHash, ConstraintF>>::ParametersVar;
