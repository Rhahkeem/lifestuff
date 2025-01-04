use clap::{Args, Subcommand};
pub mod add;
pub mod diff;

#[derive(Args, Debug)]
pub struct DateOperations {
    #[command(subcommand)]
    /// Subcommand for date operations
    pub operation_type: DateOption,
}

#[derive(Subcommand, Debug)]
pub enum DateOption {
    /// Add a time period to a given date
    Add(add::Add),
    /// Diff Two Dates
    Diff(diff::Diff),
    /// Information about the ordinal date
    Ordinal,
}
