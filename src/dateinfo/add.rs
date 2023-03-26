use crate::dateinfo::common;
use clap::{Args, ValueEnum};
use time::{Duration, Month, OffsetDateTime};

#[derive(Debug, Args, Clone)]
pub struct Add {
    #[clap(help = "Date to add time period to")]
    date: String,
    val: i64,
    #[clap(help = "Time period to add to date", required = true)]
    period: TimePeriod,
}

fn update_years(in_date: &OffsetDateTime, years: i32) -> OffsetDateTime {
    in_date
        .replace_year(in_date.year() + years)
        .unwrap_or(OffsetDateTime::now_utc())
}

fn update_months(in_date: &OffsetDateTime, months: i32) -> OffsetDateTime {
    let mut whole_years = months / 12;
    let leftovers = (months % 12) as u8;
    let current_month = u8::from(in_date.month());
    let month_idx = if current_month + leftovers > 12 {
        whole_years += 1;
        leftovers - (12 - current_month)
    } else {
        current_month + leftovers
    };

    update_years(in_date, whole_years)
        .replace_month(Month::try_from(month_idx).unwrap())
        .unwrap_or(OffsetDateTime::now_utc())
}

pub fn do_add_date(add_args: &Add, verbose: bool) {
    if verbose {
        println!("{:?}", add_args)
    }

    let mut in_date = common::get_date_fromt_string_arg(Some(&add_args.date), verbose);

    in_date.initial_utc = match &add_args.period {
        TimePeriod::Years => update_years(&in_date.initial_utc, add_args.val as i32),
        TimePeriod::Months => update_months(&in_date.initial_utc, add_args.val as i32),
        TimePeriod::Weeks => in_date.initial_utc + Duration::weeks(add_args.val),
        TimePeriod::Days => in_date.initial_utc + Duration::days(add_args.val),
        TimePeriod::Hours => in_date.initial_utc + Duration::hours(add_args.val),
        TimePeriod::Minutes => in_date.initial_utc + Duration::minutes(add_args.val),
        TimePeriod::Seconds => in_date.initial_utc + Duration::seconds(add_args.val),
    };

    let (hour, minute, second) = in_date.initial_utc.to_hms();
    println!(
        "{:?} ({hour}:{minute}:{second})",
        in_date.initial_utc.to_calendar_date()
    )
}

#[derive(Debug, ValueEnum, Clone)]
enum TimePeriod {
    #[clap( aliases = ["y","yr","yrs"])]
    Years,
    #[clap( aliases = ["m"])]
    Months,
    #[clap( aliases = ["wk","wks"])]
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
