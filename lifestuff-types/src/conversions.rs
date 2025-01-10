use clap::{Args, Subcommand};

pub mod area;
pub mod distance;

#[derive(Debug, Args)]
pub struct Conversions {
    #[command(subcommand)]
    /// Conversion type
    pub convert_type: ConversionOption,
}

#[derive(Subcommand, Debug)]
pub enum ConversionOption {
    /// Area Conversions
    Area(area::AreaConversion),
    /// Distance Conversions
    Distance(distance::DistanceConversion),
}
