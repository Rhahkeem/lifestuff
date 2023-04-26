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

pub fn distance_conversions(from: &DistanceUnits, to: &DistanceUnits, val: f64) -> f64 {
    match &from {
        DistanceUnits::Yards => match &to {
            DistanceUnits::Yards => val.to_owned(),
            DistanceUnits::Inches => val * 36.0,
            DistanceUnits::Kilometres => val * 0.0009144,
            DistanceUnits::Metres => val * 0.9144,
            DistanceUnits::Miles => val * 0.0005681818,
            DistanceUnits::Feet => val * 3.0,
        },
        DistanceUnits::Inches => match &to {
            DistanceUnits::Yards => val / 36.0,
            DistanceUnits::Inches => val.to_owned(),
            DistanceUnits::Kilometres => val * 0.0000254,
            DistanceUnits::Metres => val * 0.0254,
            DistanceUnits::Miles => val * 0.00001578283,
            DistanceUnits::Feet => val / 12.0,
        },
        DistanceUnits::Kilometres => match &to {
            DistanceUnits::Yards => val / 0.0009144,
            DistanceUnits::Inches => val * 39370.1,
            DistanceUnits::Kilometres => val.to_owned(),
            DistanceUnits::Metres => val * 1000.0,
            DistanceUnits::Miles => val / 1.609344,
            DistanceUnits::Feet => val * 3280.84,
        },
        DistanceUnits::Metres => match &to {
            DistanceUnits::Yards => val / 0.9144,
            DistanceUnits::Inches => val * 39.37007874,
            DistanceUnits::Kilometres => val / 1000.0,
            DistanceUnits::Metres => val.to_owned(),
            DistanceUnits::Miles => val / 1609.344,
            DistanceUnits::Feet => val * 3.280839895,
        },
        DistanceUnits::Miles => match &to {
            DistanceUnits::Yards => val * 1760.0,
            DistanceUnits::Inches => val * 63360.0,
            DistanceUnits::Kilometres => val * 1.609344,
            DistanceUnits::Metres => val * 1609.344,
            DistanceUnits::Miles => val.to_owned(),
            DistanceUnits::Feet => val * 5280.0,
        },
        DistanceUnits::Feet => match &to {
            DistanceUnits::Yards => val / 3.0,
            DistanceUnits::Inches => val * 12.0,
            DistanceUnits::Kilometres => val / 3280.84,
            DistanceUnits::Metres => val * 0.3048,
            DistanceUnits::Miles => val / 5280.0,
            DistanceUnits::Feet => val.to_owned(),
        },
    }
}
