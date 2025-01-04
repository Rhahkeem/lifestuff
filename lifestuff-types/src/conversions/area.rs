use clap::{Args, Subcommand, ValueEnum};
use strum::Display;

#[derive(Debug, Args)]
pub struct AreaConversion {
    #[arg(long, required = true)]
    /// Unit to convert from
    pub from: AreaUnits,
    /// Value to convert
    pub value: f64,
    #[arg(long, required = true)]
    /// Unit to convert to
    pub to: Vec<AreaUnits>,
}

//noinspection SpellCheckingInspection
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
