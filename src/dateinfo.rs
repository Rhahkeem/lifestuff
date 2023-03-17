use clap::{Args, Subcommand, ValueEnum};
use std::ops::Sub;
use strum::Display;
use time::{format_description, Date, Duration, OffsetDateTime};

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

#[derive(Debug, Clone, Copy)]
pub struct DateTimeKeeper {
    pub initial_utc: OffsetDateTime,
}

#[derive(Debug, Args, Clone)]
pub struct Diff {
    #[clap(help = "Date to perform diff operations on")]
    date1: String,

    #[clap(help = "Optional date to diff with. Defaults to current date.")]
    date2: Option<String>,
    #[clap(long, required = true, display_order = 3)]
    to: Vec<DateDuration>,
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

fn set_initial_date_argument(input_date: Option<&str>, verbose: bool) -> DateTimeKeeper {
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

pub fn do_diff_date(diff_args: &Diff, verbose: bool) {
    let first_date = set_initial_date_argument(Some(&diff_args.date1), verbose);
    let second_date = set_initial_date_argument(diff_args.date2.as_deref(), verbose);
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
