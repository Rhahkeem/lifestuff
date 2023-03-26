use std::ops::Sub;
use time::{format_description, Date, Duration, OffsetDateTime};

#[derive(Debug, Clone, Copy)]
pub struct DateTimeKeeper {
    pub initial_utc: OffsetDateTime,
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

pub fn get_date_fromt_string_arg(input_date: Option<&str>, verbose: bool) -> DateTimeKeeper {
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
