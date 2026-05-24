use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RampartTest { pub id: Uuid, pub category: OwasCategory, pub scenario: String, pub expected_behavior: String }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OwasCategory { ASI01, ASI02, ASI03, ASI04, ASI05, ASI06, ASI07, ASI08, ASI09, ASI10 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RampartResult { pub test_id: Uuid, pub passed: bool, pub category: OwasCategory, pub scenario: String, pub elapsed_ms: u64, pub findings: Vec<String> }
