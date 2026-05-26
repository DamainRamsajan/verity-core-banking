use axum::{extract::{Path, State}, Json};
use uuid::Uuid;
use verity_core_api::accounts::{CreateAccountRequest, AccountResponse, TransferRequest};
use verity_core_api::common::ApiResponse;

pub async fn create_account(
    State(_state): State<()>,
    Json(req): Json<CreateAccountRequest>,
) -> Json<ApiResponse<AccountResponse>> {
    let account = AccountResponse {
        account_id: Uuid::new_v4(),
        name: req.name,
        account_type: req.account_type,
        currency: req.currency,
        balance: rust_decimal::Decimal::ZERO,
        created_at: chrono::Utc::now(),
    };
    Json(ApiResponse::ok(account))
}

pub async fn get_account(
    State(_state): State<()>,
    Path(id): Path<Uuid>,
) -> Json<ApiResponse<AccountResponse>> {
    let account = AccountResponse {
        account_id: id,
        name: "Account".into(),
        account_type: "checking".into(),
        currency: "USD".into(),
        balance: rust_decimal::Decimal::new(1000, 0),
        created_at: chrono::Utc::now(),
    };
    Json(ApiResponse::ok(account))
}

pub async fn create_transfer(
    State(_state): State<()>,
    Json(req): Json<TransferRequest>,
) -> Json<ApiResponse<AccountResponse>> {
    let account = AccountResponse {
        account_id: req.from_account,
        name: "Account".into(),
        account_type: "checking".into(),
        currency: req.currency,
        balance: rust_decimal::Decimal::new(500, 0),
        created_at: chrono::Utc::now(),
    };
    Json(ApiResponse::ok(account))
}
