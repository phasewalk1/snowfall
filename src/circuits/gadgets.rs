use super::common::windows;
use ark_crypto_primitives::crh::injective_map::{
    constraints::{PedersenCRHCompressorGadget, TECompressorGadget},
    TECompressor,
};
use ark_ed_on_bls12_381::{constraints::EdwardsVar, EdwardsProjective};

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
