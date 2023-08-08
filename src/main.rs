use anyhow::Result;
use clap::{Parser, Subcommand};

use strum::Display;
mod conversions;
mod currency;
mod dateinfo;
mod interest;

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
    /// Currency Convertsion Operations
    Currency(currency::Currency),
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let verbose = cli.verbose;
    if verbose {
        println!("CLI Args: {:?}", cli);
    }
    match &cli.command {
        Commands::Convert(args) => {
            let x = conversions::perform_conversion(args);
            if x.is_err() {
                println!("{:?}", x.as_ref().err());
            }
            x
        }
        Commands::Dates(args) => dateinfo::handle_date_operations(args, verbose),
        Commands::Interest(args) => interest::handle_interest_calculations(args, verbose),
        Commands::Currency(args) => currency::handle_currency_opertions(args, verbose),
    }
}
