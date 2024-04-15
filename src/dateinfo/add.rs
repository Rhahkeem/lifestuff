use crate::dateinfo;
use anyhow::Result;
use clap::{Args, ValueEnum};
use time::Duration;

#[derive(Debug, Args, Clone)]
pub struct Add {
    #[clap(help = "Date to add time period to", long)]
    date: Option<String>,
    val: i32,
    #[clap(help = "Time period to add to date", required = true)]
    period: TimePeriod,
}

pub fn do_add_date(add_args: &Add, verbose: bool) -> Result<()> {
    if verbose {
        println!("Args were: {:?}", add_args)
    }

    let in_date = dateinfo::get_date_from_string_arg(add_args.date.as_deref(), verbose)?;

    let result_date = match &add_args.period {
        TimePeriod::Years => in_date.apply_year_delta(add_args.val)?,
        TimePeriod::Months => in_date.apply_month_delta(add_args.val)?,
        TimePeriod::Weeks => in_date + Duration::weeks(add_args.val.into()),
        TimePeriod::Days => in_date + Duration::days(add_args.val.into()),
        TimePeriod::Hours => in_date + Duration::hours(add_args.val.into()),
        TimePeriod::Minutes => in_date + Duration::minutes(add_args.val.into()),
        TimePeriod::Seconds => in_date + Duration::seconds(add_args.val.into()),
    };

    let (hour, minute, second) = result_date.time().as_hms();

    println!(
        "{:?} ({:0>2}:{:0>2}:{:0>2})",
        result_date, hour, minute, second
    );

    Ok(())
}

#[derive(Debug, ValueEnum, Clone)]
enum TimePeriod {
    #[clap( aliases = ["y","yr","yrs"])]
    Years,
    #[clap( aliases = ["m", "mon"])]
    Months,
    #[clap( aliases = ["w","wk","wks"])]
    Weeks,
    #[clap( aliases = ["d"])]
    Days,
    #[clap( aliases = ["h","hr","hrs"])]
    Hours,
    #[clap( aliases = ["min","mins"])]
    Minutes,
    #[clap( aliases = ["s","secs"])]
    Seconds,
}
