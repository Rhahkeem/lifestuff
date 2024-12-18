use crate::dateinfo;
use anyhow::Result;
use lifestuff_types::dateinfo::add::Add;
use lifestuff_types::dateinfo::add::TimePeriod;
use time::Duration;

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
