// use anyhow::Result;
use clap::{Args, Subcommand, ValueEnum};
use strum::Display;

#[derive(Debug, Args, Clone)]
pub struct Diff {
    #[clap(help = "Date to perform diff operations on")]
    pub date1: String,
    #[clap(help = "Optional date to diff with. Defaults to current date.")]
    pub date2: Option<String>,
    #[clap(long, required = true, display_order = 3)]
    pub to: Vec<DateDuration>,
}

#[derive(Subcommand, Debug, Display, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum DateDuration {
    #[clap(action=clap::ArgAction::SetTrue)]
    Weeks,
    #[clap(action=clap::ArgAction::SetTrue)]
    Days,
    #[clap(action=clap::ArgAction::SetTrue)]
    Hours,
    #[clap(action=clap::ArgAction::SetTrue)]
    Years,
}
