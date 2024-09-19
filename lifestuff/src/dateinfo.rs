// use clap::{Args, Subcommand};
mod add;
mod common;
mod datetimekeeper;
mod ordinal;
pub use common::*;
pub use datetimekeeper::*;
mod diff;
use anyhow::Result;
use lifestuff_types::dateinfo::{DateOperations, DateOption};

pub fn handle_date_operations(date_args: DateOperations, verbose: bool) -> Result<()> {
    match &date_args.operation_type {
        DateOption::Diff(diff_args) => diff::do_diff_date(diff_args, verbose),
        DateOption::Add(add_args) => add::do_add_date(add_args, verbose),
        DateOption::Ordinal => ordinal::handle_ordinal_operations(),
    }
}
