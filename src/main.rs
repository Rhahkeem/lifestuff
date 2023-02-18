use clap::{Args, Parser, Subcommand, ValueEnum};
use strum::Display;

#[derive(Parser)]
#[clap(
    author = "Me!",
    about = "Lifestyle library!",
    version = "0.1.0",
    long_about = "Something Something Something Daaaarksiiiide"
)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Specify unit conversion option
    Convert(Conversions),
}

#[derive(Debug, Args)]
struct Conversions {
    #[arg(value_enum)]
    from: FromUnits,
    value: f64,
    #[clap(long, required = true)]
    to: Vec<ToUnits>,
}

fn unit_conversions(from: &FromUnits, to: &ToUnits, val: &f64) -> f64 {
    match &from {
        FromUnits::Acres => match &to {
            ToUnits::Acres => return val.to_owned(),
            ToUnits::Inches => return val * 6272640.0,
            ToUnits::Km => return val * 0.004047,
            ToUnits::Metres => return val * 4046.856422,
            ToUnits::Miles => return val / 640.0,
            ToUnits::Sqft => return val * 43560.0,
        },
        FromUnits::Inches => match &to {
            ToUnits::Acres => return val / 6272640.0,
            ToUnits::Inches => val.to_owned(),
            ToUnits::Km => return val * 0.00000000064516,
            ToUnits::Metres => return val * 0.000645,
            ToUnits::Miles => return val * 0.0000000002491,
            ToUnits::Sqft => return val / 144.0,
        },
        FromUnits::Kilometres => match &to {
            ToUnits::Acres => return val * 247.105381,
            ToUnits::Inches => val * 1550003100.0062,
            ToUnits::Km => return val.to_owned(),
            ToUnits::Metres => return val * 1000000.0,
            ToUnits::Miles => return val * 0.386102,
            ToUnits::Sqft => return val * 10763910.41671,
        },
        FromUnits::Metres => match &to {
            ToUnits::Acres => return val * 0.000247,
            ToUnits::Inches => val * 1550003100.0062,
            ToUnits::Km => return val / 1000000.0,
            ToUnits::Metres => val.to_owned(),
            ToUnits::Miles => return val / 0.0000003861,
            ToUnits::Sqft => return val * 10.76391,
        },
        FromUnits::Miles => match &to {
            ToUnits::Acres => return val * 640.0,
            ToUnits::Inches => val * 4014489599.4792,
            ToUnits::Km => return val * 2.589988,
            ToUnits::Metres => val * 2589988.11,
            ToUnits::Miles => return val.to_owned(),
            ToUnits::Sqft => return val * 27878399.996383,
        },
        FromUnits::Sqft => match &to {
            ToUnits::Acres => return val / 43560.0,
            ToUnits::Inches => return val * 144.0,
            ToUnits::Km => return val / 0.000000092903,
            ToUnits::Metres => val * 0.092903,
            ToUnits::Miles => return val / 0.00000003587,
            ToUnits::Sqft => return val.to_owned(),
        },
    }
}

#[derive(Subcommand, Debug, Display, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum FromUnits {
    /// Convert From Acres
    Acres,
    /// Convert From Square Inches
    Inches,
    /// Convert From Square Kilometres
    Kilometres,
    /// Convert From Square Metres
    Metres,
    /// Convert From Square Miles
    Miles,
    /// Convert From Square Feet
    Sqft,
}

#[derive(Subcommand, Debug, Display, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum ToUnits {
    #[clap(action=clap::ArgAction::SetTrue, alias = "a")]
    /// Convert To Acres
    Acres,
    #[clap(action=clap::ArgAction::SetTrue, aliases = ["i","in"])]
    /// Convert To Square Inches
    Inches,
    #[clap(action=clap::ArgAction::SetTrue, alias = "kilometres")]
    /// Convert To Square Kilometres
    Km,
    #[clap(action=clap::ArgAction::SetTrue, alias = "m")]
    /// Convert To Square Metres
    Metres,
    #[clap(action=clap::ArgAction::SetTrue, alias = "mi")]
    /// Convert To Square Miles
    Miles,
    #[clap(action=clap::ArgAction::SetTrue, aliases = ["squarefeet","s"])]
    /// Convert To Square Feet
    Sqft,
}

fn conversion_prep(from: &FromUnits, to: &Vec<ToUnits>, val: &f64) {
    for unit in to {
        let conversion = unit_conversions(from, unit, val);
        println!("{val} {from} = {conversion} {unit}");
    }
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Convert(args) => {
            conversion_prep(&args.from, &args.to, &args.value);
        }
    }
}

// compound interest
