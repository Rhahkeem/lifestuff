use crate::dateinfo::common;
use clap::{Args, Subcommand, ValueEnum};
use strum::Display;

#[derive(Debug, Args, Clone)]
pub struct Diff {
    #[clap(help = "Date to perform diff operations on")]
    date1: String,
    #[clap(help = "Optional date to diff with. Defaults to current date.")]
    date2: Option<String>,
    #[clap(long, required = true, display_order = 3)]
    to: Vec<DateDuration>,
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

fn do_output_format(breakdown: i64, duration: &str) -> String {
    let duration_display = if breakdown == 1 {
        duration.strip_suffix("s").unwrap()
    } else {
        duration
    };

    format!("{breakdown} full {duration_display}")
}

pub fn do_diff_date(diff_args: &Diff, verbose: bool) {
    let first_date = common::get_date_fromt_string_arg(Some(&diff_args.date1), verbose);
    let second_date = common::get_date_fromt_string_arg(diff_args.date2.as_deref(), verbose);
    if verbose {
        println!(
            "Doing a date diff with {:?} and {:?}",
            first_date, second_date
        );
    }
    let date_diff = (first_date - second_date).abs();
    for option in &diff_args.to {
        let duration_type = option.to_string();
        let output = match option {
            DateDuration::Days => do_output_format(date_diff.whole_days(), &duration_type),
            DateDuration::Hours => do_output_format(date_diff.whole_hours(), &duration_type),
            DateDuration::Weeks => do_output_format(date_diff.whole_weeks(), &duration_type),
            DateDuration::Years => do_output_format(date_diff.whole_days() / 365, &duration_type),
        };

        println!("{output}");
    }
}
