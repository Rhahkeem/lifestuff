use anyhow::Result;
use clap::{Parser, Subcommand};

use strum::Display;
mod conversions;
mod currency;
mod dateinfo;
mod errors;
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
    /// DuckDuckGo Address
    DdgAddress, // New CLI option added here
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
        Commands::DdgAddress => {
            // Handle the ddg_address command here
            println!("DuckDuckGo Address command executed.");
            Ok(())
        }
    };

    if let Err(e) = da_answer {
        println!("{e}");
    }

    Ok(())
}
