pub mod compiler;
pub mod product;
pub mod temporal;
pub mod templates;
pub mod errors;

pub use compiler::AslProductCompiler;
pub use product::BankingProduct;
pub use temporal::TemporalContract;
pub use templates::{checking_account, savings_account, loan_product};
pub use errors::ProductError;
