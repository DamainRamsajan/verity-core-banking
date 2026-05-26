use axum::{extract::{Path, State}, Json};
use uuid::Uuid;
use verity_core_api::agents::{AgentResponse, AgentBoundaryRequest, AgentActivityResponse};
use verity_core_api::common::ApiResponse;

pub async fn list_agents(
    State(_state): State<()>,
) -> Json<ApiResponse<Vec<AgentResponse>>> {
    let agents = vec![
        AgentResponse {
            agent_id: Uuid::new_v4(),
            name: "Payment Agent #1".into(),
            agent_type: "payment".into(),
            status: "active".into(),
            trust_level: "trusted".into(),
            capability_count: 3,
        },
        AgentResponse {
            agent_id: Uuid::new_v4(),
            name: "Fraud Agent #3".into(),
            agent_type: "fraud".into(),
            status: "active".into(),
            trust_level: "verified".into(),
            capability_count: 2,
        },
    ];
    Json(ApiResponse::ok(agents))
}

pub async fn get_agent(
    State(_state): State<()>,
    Path(id): Path<Uuid>,
) -> Json<ApiResponse<AgentResponse>> {
    let agent = AgentResponse {
        agent_id: id,
        name: "Agent".into(),
        agent_type: "generic".into(),
        status: "active".into(),
        trust_level: "verified".into(),
        capability_count: 1,
    };
    Json(ApiResponse::ok(agent))
}

pub async fn set_boundaries(
    State(_state): State<()>,
    Path(_id): Path<Uuid>,
    Json(_req): Json<AgentBoundaryRequest>,
) -> Json<ApiResponse<AgentResponse>> {
    let agent = AgentResponse {
        agent_id: _id,
        name: "Agent".into(),
        agent_type: "generic".into(),
        status: "active".into(),
        trust_level: "verified".into(),
        capability_count: 1,
    };
    Json(ApiResponse::ok(agent))
}

pub async fn agent_activity(
    State(_state): State<()>,
    Path(_id): Path<Uuid>,
) -> Json<ApiResponse<Vec<AgentActivityResponse>>> {
    let activities = vec![
        AgentActivityResponse {
            event_id: Uuid::new_v4(),
            agent_id: _id,
            action: "debit".into(),
            amount: Some(rust_decimal::Decimal::new(250, 0)),
            risk_score: 0.05,
            within_boundary: true,
            timestamp: chrono::Utc::now(),
        },
    ];
    Json(ApiResponse::ok(activities))
}
