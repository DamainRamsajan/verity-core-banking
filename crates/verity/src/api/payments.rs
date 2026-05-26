use axum::{extract::State, Json};
use verity_core_api::payments::{PaymentRequest, PaymentResponse};
use verity_core_api::common::ApiResponse;

pub async fn create_payment(
    State(_state): State<()>,
    Json(_req): Json<PaymentRequest>,
) -> Json<ApiResponse<PaymentResponse>> {
    let payment = PaymentResponse {
        payment_id: uuid::Uuid::new_v4(),
        status: "accepted".into(),
        rail_reference: Some(format!("PAY-{}", uuid::Uuid::new_v4())),
        timestamp: chrono::Utc::now(),
    };
    Json(ApiResponse::ok(payment))
}

pub async fn list_payments(
    State(_state): State<()>,
) -> Json<ApiResponse<Vec<PaymentResponse>>> {
    Json(ApiResponse::ok(vec![]))
}
