use clap::{Args, Subcommand};
use time::Date;

#[derive(Debug, Args)]
pub struct MortgageCommand {
    #[command(subcommand)]
    pub action: MortgageAction,
}

#[derive(Debug, Subcommand)]
pub enum MortgageAction {
    /// Initialize a new mortgage
    Init(InitArgs),
    /// Record a payment
    Payment(PaymentArgs),
    /// Show current mortgage status
    Status,
    /// Show payment history
    History(HistoryArgs),
    /// Refinance current mortgage
    Refinance(RefinanceArgs),
    /// Sync data with cloud
    Sync,
}

#[derive(Debug, Args)]
pub struct InitArgs {
    /// Principal amount (e.g., 250000.00)
    #[arg(long, short)]
    pub principal: f64,

    /// Annual interest rate as percentage (e.g., 3.75)
    #[arg(long, short)]
    pub rate: f64,

    /// Term in years (e.g., 30)
    #[arg(long, short)]
    pub term: u32,

    /// Monthly payment amount (e.g., 1200.00)
    #[arg(long, short = 'm')]
    pub payment: f64,

    /// Start date (YYYY-MM-DD format)
    #[arg(long, short, value_parser = parse_date)]
    pub start_date: Date,

    /// Start date (YYYY-MM-DD format)
    #[arg(long, short, value_parser = parse_date)]
    pub end_date: Date,

    /// Optional notes
    #[arg(long)]
    pub notes: Option<String>,
}

#[derive(Debug, Args)]
pub struct PaymentArgs {
    /// Payment date (YYYY-MM-DD format)
    #[arg(long, short, value_parser = parse_date)]
    pub date: String,

    /// Scheduled payment amount (defaults to monthly payment)
    #[arg(long)]
    pub amount: Option<f64>,

    /// Additional principal payment
    #[arg(long, short, default_value = "0.00")]
    pub extra: f64,

    /// Optional notes for this payment
    #[arg(long)]
    pub notes: Option<String>,
}

#[derive(Debug, Args)]
pub struct HistoryArgs {
    /// Number of recent payments to show
    #[arg(long, short, default_value = "12")]
    pub last: u32,

    /// Show all payments (overrides --last)
    #[arg(long, short)]
    pub all: bool,
}

#[derive(Debug, Args)]
pub struct RefinanceArgs {
    /// New annual interest rate as percentage
    #[arg(long)]
    pub new_rate: f64,

    /// New term in years
    #[arg(long)]
    pub new_term: u32,

    /// New monthly payment amount
    #[arg(long)]
    pub new_payment: f64,

    /// Refinance date (YYYY-MM-DD format, defaults to today)
    #[arg(long, value_parser = parse_date)]
    pub date: Option<Date>,

    /// Optional notes about refinancing
    #[arg(long)]
    pub notes: Option<String>,
}

// Helper function to parse dates - simple approach consistent with codebase patterns
fn parse_date(s: &str) -> Result<Date, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 3 {
        return Err("Invalid date format, expected YYYY-MM-DD".into());
    }

    let year: i32 = parts[0].parse()?;
    let month = time::Month::try_from(parts[1].parse::<u8>()?)?;
    let day: u8 = parts[2].parse()?;

    Ok(Date::from_calendar_date(year, month, day)?)
}
