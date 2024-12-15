use clap::{Args, Subcommand};

pub mod convert;

#[derive(Args, Debug)]
pub struct DDGOperations {
    #[command(subcommand)]
    pub operation_type: DDGOption,
}

#[derive(Subcommand, Debug)]
pub enum DDGOption {
    /// Generates a Duckduckgo email alias address
    Generate,
    /// Converts a regular email address to be used by Duckduckgo as a recipient
    Convert(convert::DDGConvert),
}
