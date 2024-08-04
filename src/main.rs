use anyhow::Result;
use clap::{Parser, Subcommand};

use strum::Display;
mod conversions;
mod currency;
mod dateinfo;
mod interest;
mod mileage;

#[derive(Parser, Debug)]
#[clap(
    author = "Me!",
    about = "Lifestyle library!",
    version = "0.2.0",
    long_about = "Something Something Something Daaaarksiiiide"
)]
struct Cli {
    #[arg(short, long, global=true, action=clap::ArgAction::SetTrue)]
    verbose: bool,
    #[arg(long, help = "DuckDuckGo address for API calls")]
    ddg_address: Option<String>,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Display, Debug)]
enum Commands {
    /// Unit conversions
    Convert(conversions::Conversions),
    /// Date Operations
    Dates(dateinfo::DateOperations),
    /// Interest Calculations
    Interest(interest::Interest),
    /// Currency Conversion Operations
    Currency(currency::Currency),
    /// Mileage Calculations
    Mileage(mileage::Mileage),
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let verbose = cli.verbose;
    if verbose {
        println!("CLI Args: {:?}", cli);
    }
    let da_answer = match &cli.command {
        Commands::Convert(args) => conversions::perform_conversion(args),
        Commands::Dates(args) => dateinfo::handle_date_operations(args, verbose),
        Commands::Interest(args) => interest::handle_interest_calculations(args, verbose),
        Commands::Currency(args) => currency::handle_currency_operations(args, verbose),
        Commands::Mileage(args) => mileage::handle_mileage_operations(args, verbose),
    };

    if da_answer.is_err() {
        println!("{}", da_answer.unwrap_err());
    }

    Ok(())
}
