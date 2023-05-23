use clap::{Args, Subcommand};
mod add;
mod common;
mod datetimekeeper;
pub use common::*;
pub use datetimekeeper::*;
mod diff;

#[derive(Args, Debug)]
pub struct DateOperations {
    #[command(subcommand)]
    operation_type: DateOption,
}

#[derive(Subcommand, Debug)]
enum DateOption {
    ///Add a time period to a given date
    Add(add::Add),
    ///Diff Two Dates
    Diff(diff::Diff),
}

pub fn handle_date_operations(date_args: &DateOperations, verbose: bool) {
    match &date_args.operation_type {
        DateOption::Diff(diff_args) => {
            diff::do_diff_date(diff_args, verbose);
        }
        DateOption::Add(add_args) => {
            add::do_add_date(add_args, verbose);
        }
    }
}
