use clap::{Parser, Subcommand};
use strum::Display;

pub mod conversions;
pub mod currency;
pub mod dateinfo;
pub mod ddg;
pub mod interest;
pub mod mileage;

pub fn parse() -> Cli {
    Cli::parse()
}

#[derive( Parser, Debug)]
#[command(
    author = "Me!",
    about = "Lifestyle library!",
    version = "0.2.0",
    long_about = "Something Something Something Daaaarksiiiide"
)]
pub struct Cli {
    #[arg(short, long, global=true, action=clap::ArgAction::SetTrue)]
    pub verbose: bool,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Display, Debug)]
pub enum Commands {
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
    DDG(ddg::DDGOperations),
}
