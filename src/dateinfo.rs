use clap::{Args, Subcommand, ValueEnum};
use std::ops::Sub;
use strum::Display;
use time::{format_description, Date, Duration, Month, OffsetDateTime};

#[derive(Subcommand, Debug, Display, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum DateDuration {
    #[clap(action=clap::ArgAction::SetTrue)]
    Weeks,
    #[clap(action=clap::ArgAction::SetTrue)]
    Days,
    #[clap(action=clap::ArgAction::SetTrue)]
    Hours,
    #[clap(action=clap::ArgAction::SetTrue)]
    Years,
}

#[derive(Debug, Clone, Copy)]
struct DateTimeKeeper {
    initial_utc: OffsetDateTime,
}

#[derive(Debug, Args, Clone)]
struct Diff {
    #[clap(help = "Date to perform diff operations on")]
    date1: String,

    #[clap(help = "Optional date to diff with. Defaults to current date.")]
    date2: Option<String>,
    #[clap(long, required = true, display_order = 3)]
    to: Vec<DateDuration>,
}

#[derive(Debug, Args, Clone)]
struct Add {
    #[clap(help = "Date to add time period to")]
    date: String,
    val: i64,
    #[clap(help = "Time period to add to date", required = true)]
    period: TimePeriod,
}

#[derive(Debug, ValueEnum, Clone)]
enum TimePeriod {
    Years,
    Months,
    Weeks,
    Days,
    Hours,
    Minutes,
    Seconds,
}

fn do_output_format(breakdown: i64, duration: &str) -> String {
    let duration_display = if breakdown == 1 {
        duration.strip_suffix("s").unwrap()
    } else {
        duration
    };

    format!("{breakdown} full {duration_display}")
}
// impl fmt::Debug for DateTimeKeeper {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//             write!(
//                 f,"{:?}", self.initial_utc.to_calendar_date()
//             )
//         }
//     // fn print_info(&self) -> String {
//     //     format!("{:?}", self.initial_utc.to_calendar_date())
//     // }
// }

fn make_datekeeper(initial: Option<(i32, u8, u8)>, verbose: bool) -> DateTimeKeeper {
    if initial.is_some() {
        let date_format = format_description::parse("[year]-[month]-[day]").unwrap();
        let the_date = format!(
            "{:04}-{:02}-{:02}",
            initial.unwrap().0,
            initial.unwrap().1,
            initial.unwrap().2
        );

        if verbose {
            println!("The date is {:?}", the_date);
        }
        DateTimeKeeper {
            initial_utc: OffsetDateTime::now_utc()
                .replace_date(Date::parse(&the_date, &date_format).unwrap()),
        }
    } else {
        DateTimeKeeper {
            initial_utc: OffsetDateTime::now_utc(),
        }
    }
}

impl Sub for DateTimeKeeper {
    type Output = Duration;

    fn sub(self, other: Self) -> Self::Output {
        self.initial_utc - other.initial_utc
    }
}

fn parse_input_date(input: &str, verbose: bool) -> Result<(i32, u8, u8), &'static str> {
    let parse_date_result = i32::from_str_radix(input, 10);
    if parse_date_result.is_ok() {
        let foo = parse_date_result.as_ref().ok().unwrap();
        if verbose {
            println!("Foo is now {}", foo);
        }
        return Ok((foo / 10000, ((foo / 100) % 100) as u8, (foo % 100) as u8));
    }
    if input.contains("-") || input.contains("/") {
        let (tokens, year_idx, month_idx, date_idx) = match input.contains("-") {
            true => (input.split("-").collect::<Vec<&str>>(), 0, 1, 2),
            false => (input.split("/").collect::<Vec<&str>>(), 2, 1, 0),
        };
        if verbose {
            println!("The tokens were {:?}", tokens);
        }
        if tokens.len() == 3 {
            let year = i32::from_str_radix(tokens[year_idx], 10);
            if year.is_err() {
                return Err("Error handling year parsing");
            }

            if verbose {
                println!("Year is {:?}", year);
            }
            let month = u8::from_str_radix(tokens[month_idx], 10);
            if month.is_err() {
                return Err("Error handling month parsing");
            }
            if verbose {
                println!("Month is {:?}", month);
            }
            let date = u8::from_str_radix(tokens[date_idx], 10);
            if date.is_err() {
                //todo handle parse error
                return Err("Error handling date parsing");
            }
            if verbose {
                println!("Date is {:?}", date);
            }
            return Ok((
                if year.as_ref().unwrap() < &100 {
                    year.unwrap() + 2000
                } else {
                    year.unwrap()
                },
                month.unwrap(),
                date.unwrap(),
            ));
        }
    }

    return Err("Error handling year parsing");
}

fn get_date_fromt_string_arg(input_date: Option<&str>, verbose: bool) -> DateTimeKeeper {
    if input_date.is_some() {
        let parsed_date = parse_input_date(input_date.unwrap(), verbose);
        if parsed_date.is_ok() {
            make_datekeeper(Some(parsed_date.unwrap()), verbose)
        } else {
            println!(
                "Encountered an error parsing input date '{:?}'; Err:{:?}",
                input_date.unwrap(),
                parsed_date.err().unwrap()
            );
            make_datekeeper(None, verbose)
        }
    } else {
        make_datekeeper(None, verbose)
    }
}

fn do_diff_date(diff_args: &Diff, verbose: bool) {
    let first_date = get_date_fromt_string_arg(Some(&diff_args.date1), verbose);
    let second_date = get_date_fromt_string_arg(diff_args.date2.as_deref(), verbose);
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

#[derive(Args, Debug)]
pub struct DateOperations {
    #[command(subcommand)]
    operation_type: DateOption,
}

#[derive(Subcommand, Debug)]
enum DateOption {
    ///Add a time period to a given date
    Add(Add),
    ///Diff Two Dates
    Diff(Diff),
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

fn do_add_date(add_args: &Add, verbose: bool) {
    if verbose {
        println!("{:?}", add_args)
    }

    let mut in_date = get_date_fromt_string_arg(Some(&add_args.date), verbose);

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
pub fn handle_date_operations(date_args: &DateOperations, verbose: bool) {
    match &date_args.operation_type {
        DateOption::Diff(diff_args) => {
            do_diff_date(diff_args, verbose);
        }
        DateOption::Add(add_args) => {
            do_add_date(add_args, verbose);
        }
    }
}
