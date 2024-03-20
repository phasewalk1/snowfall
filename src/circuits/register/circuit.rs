use super::{Root, SnowfallPath};
use crate::circuits::common::{
    CommitmentVar, ConstraintF, Gamma, GammaVar, PublicKey, PublicKeyVar, RootVar, SnowfallPathVar,
};
use ark_ed_on_bls12_381::EdwardsAffine;
use ark_r1cs_std::prelude::*;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
// use ark_crypto_primitives::prf::blake2s::constraints::evaluate_blake2s;

/// The `Registration` circuit is used to register voters in Snowfall
pub struct RegistrationCircuit {
    // Public inputs
    pub root: Root,
    pub comm: EdwardsAffine,
    // Private witness
    pub pk: Option<PublicKey>,
    pub gamma: Option<Gamma>,
    pub path: Option<SnowfallPath>,
}

impl ConstraintSynthesizer<ConstraintF> for RegistrationCircuit {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<ConstraintF>,
    ) -> ark_relations::r1cs::Result<()> {
        // ---------- Allocate variables ----------

        // ---------- Public inputs ----------
        let root = RootVar::new_input(cs.clone(), || Ok(&self.root))?;

        let comm =
            CommitmentVar::new_input(ark_relations::ns!(cs, "comm"), || Ok(self.comm.clone()))?;

        // ---------- Private witness ----------
        let pk = PublicKeyVar::new_variable(
            ark_relations::ns!(cs, "pk"),
            || Ok(self.pk.unwrap()),
            AllocationMode::Witness,
        )?;

        let path = SnowfallPathVar::new_witness(ark_relations::ns!(cs, "path"), || {
            self.path.ok_or(SynthesisError::AssignmentMissing)
        })?;

        let gamma = GammaVar::new_witness(ark_relations::ns!(cs, "gamma"), || {
            self.gamma.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // ---------- Main constraints ----------

        // Ensure comm is a valid Pedersen commitment to pk with the given gamma

        // Check that the path is a valid path in the Merkle tree

        // Compute nullifier
        // let nullifier = evaluate_blake2s(comm.to_bytes()?, gamma.to_bytes()?)?;
        todo!();
    }
}
