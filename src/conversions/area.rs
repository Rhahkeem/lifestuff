use clap::{Args, Subcommand, ValueEnum};
use strum::Display;

#[derive(Debug, Args)]
pub struct AreaConversion {
    #[arg(long, required = true)]
    pub from: AreaUnits,
    pub value: f64,
    #[arg(long, required = true)]
    pub to: Vec<AreaUnits>,
}

#[derive(Subcommand, Debug, Display, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum AreaUnits {
    #[clap(action=clap::ArgAction::SetTrue, alias = "a")]
    /// Convert Using Acres
    Acres,
    #[clap(action=clap::ArgAction::SetTrue, name="inches", aliases = ["i","in"])]
    /// Convert Using Square Inches
    SqInches,
    #[clap(action=clap::ArgAction::SetTrue, name="km", aliases =["sqkm"])]
    /// Convert Using Square Kilometres
    SqKilometres,
    #[clap(action=clap::ArgAction::SetTrue, name="metres",aliases = ["sqm","m"])]
    /// Convert Using Square Metres
    SquareMetres,
    #[clap(action=clap::ArgAction::SetTrue, name="miles", aliases = ["sqmi","mi"])]
    /// Convert Using Square Miles
    SquareMiles,
    #[clap(action=clap::ArgAction::SetTrue, name="sqft", aliases = ["squarefeet","s"])]
    /// Convert Using Square Feet
    SquareFeet,
}

pub fn area_conversions(from: &AreaUnits, to: &AreaUnits, val: f64) -> f64 {
    match &from {
        AreaUnits::Acres => match &to {
            AreaUnits::Acres => val.to_owned(),
            AreaUnits::SqInches => val * 6272640.0,
            AreaUnits::SqKilometres => val * 0.004047,
            AreaUnits::SquareMetres => val * 4046.856422,
            AreaUnits::SquareMiles => val / 640.0,
            AreaUnits::SquareFeet => val * 43560.0,
        },
        AreaUnits::SqInches => match &to {
            AreaUnits::Acres => val / 6272640.0,
            AreaUnits::SqInches => val.to_owned(),
            AreaUnits::SqKilometres => val * 0.00000000064516,
            AreaUnits::SquareMetres => val * 0.000645,
            AreaUnits::SquareMiles => val * 0.0000000002491,
            AreaUnits::SquareFeet => val / 144.0,
        },
        AreaUnits::SqKilometres => match &to {
            AreaUnits::Acres => val * 247.105381,
            AreaUnits::SqInches => val * 1550003100.0062,
            AreaUnits::SqKilometres => val.to_owned(),
            AreaUnits::SquareMetres => val * 1000000.0,
            AreaUnits::SquareMiles => val * 0.386102,
            AreaUnits::SquareFeet => val * 10763910.41671,
        },
        AreaUnits::SquareMetres => match &to {
            AreaUnits::Acres => val * 0.000247,
            AreaUnits::SqInches => val * 1550.0031000062,
            AreaUnits::SqKilometres => val / 1000000.0,
            AreaUnits::SquareMetres => val.to_owned(),
            AreaUnits::SquareMiles => val / 2589988.11,
            AreaUnits::SquareFeet => val * 10.76391,
        },
        AreaUnits::SquareMiles => match &to {
            AreaUnits::Acres => val * 640.0,
            AreaUnits::SqInches => val * 4014489600.0,
            AreaUnits::SqKilometres => val * 2.589988,
            AreaUnits::SquareMetres => val * 2589988.11,
            AreaUnits::SquareMiles => val.to_owned(),
            AreaUnits::SquareFeet => val * 27878400.0,
        },
        AreaUnits::SquareFeet => match &to {
            AreaUnits::Acres => val * 0.0009290304,
            AreaUnits::SqInches => val * 144.0,
            AreaUnits::SqKilometres => val * 0.000000092903,
            AreaUnits::SquareMetres => val * 0.09290304,
            AreaUnits::SquareMiles => val / 27878400.0,
            AreaUnits::SquareFeet => val.to_owned(),
        },
    }
}
