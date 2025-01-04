use clap::Args;

#[derive(Debug, Args, Clone)]
pub struct Interest {
    #[clap(
        help = "Principal left on mortgage",
        short,
        long,
        allow_negative_numbers = false,
        required = true
    )]
    /// Principal left on mortgage
    pub principal: f32,
    #[clap(
        help = "Interest rate (%)",
        short,
        long,
        allow_negative_numbers = false,
        required = true
    )]
    /// Current Interest rate (%)
    pub interest_rate: f32,
    #[clap(
        help = "Monthly payment amount",
        long,
        allow_negative_numbers = false,
        required = true
    )]
    /// Monthly payment amount
    pub repayment: f32,
    #[clap(
        help = "Max annual repayment percentage (%)",
        short,
        long,
        allow_negative_numbers = false,
        required = true,
        visible_alias = "annual-limit"
    )]
    /// Maximum annual repayment percentage (%) allowed
    pub max_repayment_pct: Option<u8>,
    #[clap(
        help = "Max annual supplementary downpayment ",
        short,
        conflicts_with = "max_repayment_pct",
        long,
        required = true,
        allow_negative_numbers = false,
        visible_alias = "annual-downpayment"
    )]
    /// Maximum annual supplementary downpayment allowed
    pub annual_downpayment: Option<f32>,
    #[clap(
        help = "Mortgage calculation end date (dd/mm/yyyy)",
        short,
        long,
        allow_negative_numbers = false
    )]
    /// Mortgage calculation end date (dd/mm/yyyy)
    pub end_date: String,
}
