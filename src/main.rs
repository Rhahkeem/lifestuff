use clap::{Parser, Subcommand};
use strum::Display;
mod conversions;
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
}

fn main() {
    let cli = Cli::parse();
    let verbose = cli.verbose;
    if verbose {
        println!("CLI Args: {:?}", cli);
    }
    match &cli.command {
        Commands::Convert(args) => conversions::perform_conversion(args, verbose),

        Commands::Dates(args) => dateinfo::handle_date_operations(args, verbose),

        Commands::Interest(args) => interest::handle_interest_calculations(args, verbose),
    }
}

// compound interest
