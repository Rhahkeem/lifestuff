use clap::{Args, Subcommand};
use serde::{Deserialize, Serialize};

/// Mortgage management operations
#[derive(Debug, Args, Clone)]
pub struct MortgageCommand {
    #[command(subcommand)]
    pub operation: MortgageOperation,

    #[arg(
        long,
        global = true,
        help = "API endpoint URL (e.g., https://api.example.com). Falls back to LIFESTUFF_API_ENDPOINT env var, then http://localhost:8787"
    )]
    pub endpoint: Option<String>,
}

#[derive(Debug, Subcommand, Clone)]
pub enum MortgageOperation {
    /// Record a mortgage payment
    Payment(PaymentArgs),
    /// Get current mortgage status
    Status,
    /// Get payment history
    History,
    /// Get interest posting history
    InterestHistory,
    /// Add an interest posting
    Interest(InterestPostingArgs),
}

// CLI argument structs

/// Arguments for recording a payment
#[derive(Debug, Args, Clone)]
pub struct PaymentArgs {
    #[arg(
        short = 'D',
        long = "default",
        conflicts_with = "monthly_payment",
        help = "Use mortgage's default monthly payment"
    )]
    pub use_default: bool,

    #[arg(
        short = 'm',
        long = "monthly",
        help = "Monthly payment amount (overrides default). If neither -D nor -m is specified, monthly payment is set to 0"
    )]
    pub monthly_payment: Option<f64>,

    #[arg(short = 'o', long, help = "Overpayment/additional payment amount")]
    pub overpayment: Option<f64>,

    #[arg(short, long, help = "Payment date in DD/MM/YYYY format")]
    pub date: String,

    #[arg(short, long, help = "Optional note for the payment")]
    pub note: Option<String>,
}

/// Arguments for adding an interest posting
#[derive(Debug, Args, Clone)]
pub struct InterestPostingArgs {
    #[arg(short, long, help = "Interest amount to post")]
    pub amount: f64,

    #[arg(short, long, help = "Posting date in DD/MM/YYYY format")]
    pub date: String,

    #[arg(short, long, help = "Optional note for the posting")]
    pub note: Option<String>,
}

// Request/Response types for API communication

#[derive(Debug, Serialize)]
pub struct PaymentRequest {
    pub payment_date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_payment: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_payment: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct InterestPostingRequest {
    pub posting_date: String,
    pub interest_amount: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_balance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

// Response types

#[derive(Debug, Deserialize, Serialize)]
pub struct MortgageSummary {
    pub id: Option<i64>,
    pub initial_principal: f64,
    pub interest_rate: f64,
    pub term_years: i32,
    pub monthly_payment: f64,
    pub start_date: String,
    pub end_date: Option<String>,
    pub status: String,
    pub previous_mortgage_id: Option<i64>,
    pub notes: Option<String>,
    pub created_at: Option<String>,
    pub payments_made: i32,
    pub total_paid: f64,
    pub extra_payments: f64,
    pub current_balance: f64,
    pub last_payment_date: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PaymentRecord {
    pub id: Option<i64>,
    pub mortgage_id: i64,
    pub payment_date: String,
    pub scheduled_payment: f64,
    pub additional_payment: f64,
    pub total_payment: f64,
    pub principal_portion: f64,
    pub interest_portion: f64,
    pub remaining_balance: f64,
    pub payment_number: Option<i32>,
    pub notes: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InterestPosting {
    pub id: Option<i64>,
    pub mortgage_id: i64,
    pub posting_date: String,
    pub interest_amount: f64,
    pub remaining_balance: f64,
    pub notes: Option<String>,
    pub created_at: Option<String>,
}
