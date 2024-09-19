use clap::Args;

///Convert from one currency to another
#[derive(Debug, Args, Clone)]
pub struct Currency {
    #[clap(short, long, help = "Currency to convert from")]
    pub from: String,
    #[clap(
        short,
        long,
        allow_negative_numbers = false,
        help = "Amount to convert"
    )]
    pub amt: f64,
    #[clap(short, long, help = "Currency to convert to")]
    pub to: Vec<String>,
}
