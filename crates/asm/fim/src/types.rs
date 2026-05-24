use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterChange { pub parameter_name: String, pub old_value: String, pub new_value: String, pub authorized: bool }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyAuthorization { pub policy_id: uuid::Uuid, pub parameter: String, pub signature: Vec<u8>, pub approved_by: String }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvariantCheck { pub parameter: String, pub satisfied: bool, pub evidence: Option<String> }
