use clap::Args;

/// Convert from one currency to another
#[derive(Debug, Args, Clone)]
pub struct Currency {
    #[clap(short, long, help = "Currency to convert from")]
    /// Currency to convert from
    pub from: String,
    #[clap(
        short,
        long,
        allow_negative_numbers = false,
        help = "Amount to convert"
    )]
    /// Amount to convert
    pub amt: f64,
    #[clap(short, long, help = "Currency to convert to")]
    /// Currency to convert to
    pub to: Vec<String>,
    #[arg(
        long,
        global = true,
        help = "API endpoint URL (e.g., https://api.example.com). Falls back to LIFESTUFF_API_ENDPOINT env var, then http://localhost:8787"
    )]
    pub endpoint: Option<String>,
}
