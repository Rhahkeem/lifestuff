use crate::dateinfo::common;
use anyhow::Result;
use lifestuff_types::dateinfo::diff::{DateDuration, Diff};

fn do_output_format(breakdown: i64, duration: &str) -> String {
    let duration_display = if breakdown == 1 {
        duration.strip_suffix("s").unwrap()
    } else {
        duration
    };

    format!("{breakdown} full {duration_display}")
}

pub fn do_diff_date(diff_args: &Diff, verbose: bool) -> Result<()> {
    let first_date = common::get_date_from_string_arg(Some(&diff_args.date1), verbose)?;
    let second_date = common::get_date_from_string_arg(diff_args.date2.as_deref(), verbose)?;
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

    Ok(())
}
