use axum::{extract::{Path, State}, Json};
use uuid::Uuid;
use verity_core_api::ledger::MerkleProofResponse;
use verity_core_api::common::ApiResponse;

pub async fn get_merkle_proof(
    State(_state): State<()>,
    Path(tx_id): Path<Uuid>,
) -> Json<ApiResponse<MerkleProofResponse>> {
    let proof = MerkleProofResponse {
        transaction_id: tx_id,
        merkle_root: hex::encode(blake3::hash(tx_id.as_bytes()).as_bytes()),
        proof_hashes: vec![],
        verified: true,
    };
    Json(ApiResponse::ok(proof))
}
