use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use ark_groth16::Groth16;
use ark_bn254::Bn254;
use ark_snark::{CircuitSpecificSetupSNARK, SNARK};
use ark_serialize::CanonicalSerialize;

use super::types::{PsiComplianceProof, PsiRequest};
use super::errors::PsiError;

/// A dummy circuit for demonstration – replaced with real regulatory logic later.
mod dummy_circuit {
    use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
    use ark_bn254::Fr;

    pub struct DummyCircuit;

    impl ConstraintSynthesizer<Fr> for DummyCircuit {
        fn generate_constraints(self, _cs: ConstraintSystemRef<Fr>) -> Result<(), SynthesisError> {
            Ok(())
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PqcSignature {
    pub classical: Vec<u8>,
    pub pqc: Option<Vec<u8>>,
}

#[derive(Debug)]
pub struct PsiEngineConfig {
    pub mpc_consensus_enabled: bool,
    pub mpc_nodes: usize,
}

impl Default for PsiEngineConfig {
    fn default() -> Self {
        Self {
            mpc_consensus_enabled: true,
            mpc_nodes: 3,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct PsiEngineStats {
    pub proofs_generated: u64,
    pub proofs_verified: u64,
}

pub struct PsiEngine {
    proofs: Arc<RwLock<HashMap<Uuid, PsiComplianceProof>>>,
    config: PsiEngineConfig,
    stats: Arc<RwLock<PsiEngineStats>>,
    // Pre‑generated proving/verifying key pair for the dummy circuit.
    _pk: ark_groth16::ProvingKey<Bn254>,
    vk: ark_groth16::VerifyingKey<Bn254>,
}

impl PsiEngine {
    pub fn new(config: PsiEngineConfig) -> Self {
        let (pk, vk) = Groth16::<Bn254>::setup(
            dummy_circuit::DummyCircuit,
            &mut rand::thread_rng(),
        )
        .expect("Failed to perform dummy Groth16 setup");
        Self {
            proofs: Arc::new(RwLock::new(HashMap::new())),
            config,
            stats: Arc::new(RwLock::new(PsiEngineStats::default())),
            _pk: pk,
            vk,
        }
    }

    #[tracing::instrument(skip(self), level = "info")]
    pub async fn generate_compliance_proof(
        &self,
        request: &PsiRequest,
        institution_id: &str,
    ) -> Result<PsiComplianceProof, PsiError> {
        if self.config.mpc_consensus_enabled {
            // In production, coordinate with mpc_nodes nodes.
        }

        let circuit = dummy_circuit::DummyCircuit;
        let proof = Groth16::<Bn254>::prove(&self._pk, circuit, &mut rand::thread_rng())
            .map_err(|e| PsiError::ProofGenerationError(e.to_string()))?;

        let mut proof_bytes = Vec::new();
        proof
            .serialize_compressed(&mut proof_bytes)
            .map_err(|e| PsiError::ProofGenerationError(e.to_string()))?;

        let merkle_root = hex::encode(blake3::hash(b"ledger_state").as_bytes());

        let psi_proof = PsiComplianceProof {
            proof_id: Uuid::new_v4(),
            regulator_id: request.regulator_id.clone(),
            institution_id: institution_id.to_string(),
            proof_data: proof_bytes,
            groth16_vk: None,
            pqc_signature: Some(PqcSignature {
                classical: vec![],
                pqc: None,
            }),
            merkle_root,
            timestamp: chrono::Utc::now(),
        };

        self.proofs.write().await.insert(psi_proof.proof_id, psi_proof.clone());
        self.stats.write().await.proofs_generated += 1;
        Ok(psi_proof)
    }

    #[tracing::instrument(skip(self), level = "info")]
    pub fn verify_proof(&self, proof: &PsiComplianceProof) -> Result<bool, PsiError> {
        use ark_serialize::CanonicalDeserialize;
        let deserialized_proof = ark_groth16::Proof::<Bn254>::deserialize_compressed(
            &proof.proof_data[..],
        )
        .map_err(|e| PsiError::ProofVerificationError(e.to_string()))?;

        let pvk = ark_groth16::prepare_verifying_key(&self.vk);
        let public_inputs: Vec<ark_bn254::Fr> = vec![];
        let result = ark_groth16::Groth16::<Bn254>::verify_proof(&pvk, &deserialized_proof, &public_inputs)
            .map_err(|e| PsiError::ProofVerificationError(e.to_string()))?;

        // Stats update is async; we use block_on to keep this function sync.
        let handle = tokio::runtime::Handle::current();
        handle.block_on(async {
            self.stats.write().await.proofs_verified += 1;
        });
        Ok(result)
    }

    pub async fn get_stats(&self) -> PsiEngineStats {
    (*self.stats.read().await).clone()
    }
}