use clap::{Args, Subcommand, ValueEnum};
use strum::Display;

#[derive(Debug, Args)]
pub struct DistanceConversion {
    #[clap(long, required = true, display_order = 1)]
    pub from: DistanceUnits,
    #[clap(display_order = 2)]
    pub value: f64,
    #[clap(long, required = true, display_order = 3)]
    pub to: Vec<DistanceUnits>,
}

#[derive(Subcommand, Debug, Display, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum DistanceUnits {
    #[clap(action=clap::ArgAction::SetTrue, aliases = ["ft","f"])]
    /// Using Feet
    Feet,
    #[clap(action=clap::ArgAction::SetTrue, aliases = ["i","in"])]
    /// Using Inches
    Inches,
    #[clap(action=clap::ArgAction::SetTrue, alias = "km")]
    /// Using Kilometres
    Kilometres,
    #[clap(action=clap::ArgAction::SetTrue, alias = "m")]
    /// Using Metres
    Metres,
    #[clap(action=clap::ArgAction::SetTrue, alias = "mi")]
    /// Using Miles
    Miles,
    #[clap(action=clap::ArgAction::SetTrue, alias = "y")]
    /// Using Yards
    Yards,
}
