use super::types::EmotionalContext;

/// Classifies transactions into emotional contexts.
pub struct EmotionClassifier;

impl EmotionClassifier {
    pub fn new() -> Self { Self }

    pub fn classify(
        &self,
        transaction_type: &str,
        _amount: Option<rust_decimal::Decimal>,
    ) -> EmotionalContext {
        match transaction_type {
            "overdraft" | "declined_payment" | "unexpected_fee" => EmotionalContext::FinancialStress,
            "flagged_transaction" | "new_device_login" | "large_transfer" => EmotionalContext::SecurityAnxiety,
            "mortgage_application" | "first_investment" | "savings_goal" => EmotionalContext::LifeMilestone,
            _ => EmotionalContext::Routine,
        }
    }
}
