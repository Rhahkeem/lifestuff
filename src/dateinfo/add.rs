use crate::dateinfo;
use clap::{Args, ValueEnum};
use time::Duration;

#[derive(Debug, Args, Clone)]
pub struct Add {
    #[clap(help = "Date to add time period to")]
    date: String,
    val: i32,
    #[clap(help = "Time period to add to date", required = true)]
    period: TimePeriod,
}

pub fn do_add_date(add_args: &Add, verbose: bool) {
    if verbose {
        println!("{:?}", add_args)
    }

    let mut in_date = dateinfo::get_date_from_string_arg(Some(&add_args.date), verbose);

    match &add_args.period {
        TimePeriod::Years => in_date.update_years(add_args.val),
        TimePeriod::Months => in_date.update_months(add_args.val),
        TimePeriod::Weeks => in_date.update_weeks(add_args.val as i64),
        TimePeriod::Days => in_date.update_days(add_args.val as i64),
        TimePeriod::Hours => in_date += Duration::hours(add_args.val.into()),
        TimePeriod::Minutes => in_date += Duration::minutes(add_args.val.into()),
        TimePeriod::Seconds => in_date += Duration::seconds(add_args.val.into()),
    };

    let (hour, minute, second) = in_date.time().as_hms();

    println!("{:?} ({:0>2}:{:0>2}:{:0>2})", in_date, hour, minute, second)
}

#[derive(Debug, ValueEnum, Clone)]
enum TimePeriod {
    #[clap( aliases = ["y","yr","yrs"])]
    Years,
    #[clap( aliases = ["m"])]
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
