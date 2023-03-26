use std::ops::Sub;
use time::{format_description, Date, Duration, OffsetDateTime, Time};

#[derive(Debug, Clone, Copy)]
pub struct DateTimeKeeper {
    pub initial_utc: OffsetDateTime,
    // date: Date,
    // time: Time,
}

fn parse_input_date(input: &str, verbose: bool) -> Result<(u32, u8, u8), &'static str> {
    let parse_date_result = u32::from_str_radix(input, 10);
    if parse_date_result.is_ok() {
        let yyyymmdd = parse_date_result.as_ref().ok().unwrap();
        if verbose {
            println!("String parsed date is: {yyyymmdd}");
        }
        if yyyymmdd < &10000000 {
            return Err("Error parsing `yyyymmdd` date format");
        }
        return Ok((
            yyyymmdd / 10000,
            ((yyyymmdd / 100) % 100) as u8,
            (yyyymmdd % 100) as u8,
        ));
    }
    if !input.contains("-") && !input.contains("/") {
        return Err("Error handling date parsing. Expected format dd/mm/yyyy or dd-mm-yyyy");
    }

    let (tokens, year_idx, month_idx, date_idx) = match input.contains("-") {
        true => (input.split("-").collect::<Vec<&str>>(), 2, 1, 0),
        false => (input.split("/").collect::<Vec<&str>>(), 2, 1, 0),
    };

    if verbose {
        println!("The tokens were {:?}", tokens);
    }

    if tokens.len() != 3 {
        return Err("Error handling date parsing. Found less than 3 tokens to parse");
    }

    let year = u32::from_str_radix(tokens[year_idx], 10);
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
    if month.as_ref().unwrap() > &12 {
        return Err("Invalid month passed; (value > 12)");
    }
    if verbose {
        println!("Month is {:?}", month);
    }
    let date = u8::from_str_radix(tokens[date_idx], 10);
    if date.is_err() {
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

pub fn get_date_fromt_string_arg(input_date: Option<&str>, verbose: bool) -> DateTimeKeeper {
    if input_date.is_some() {
        let parsed_date = parse_input_date(input_date.unwrap(), verbose);
        if parsed_date.is_ok() {
            make_datekeeper(Some(parsed_date.unwrap()), verbose)
        } else {
            println!(
                "Encountered an error parsing input date '{:?}'; Err: {:?}. Defulting to curent date instead.",
                input_date.unwrap(),
                parsed_date.err().unwrap()
            );
            make_datekeeper(None, verbose)
        }
    } else {
        make_datekeeper(None, verbose)
    }
}

fn make_datekeeper(initial: Option<(u32, u8, u8)>, verbose: bool) -> DateTimeKeeper {
    let date_time_now = OffsetDateTime::now_utc();
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
        let in_date = Date::parse(&the_date, &date_format).unwrap();
        DateTimeKeeper {
            initial_utc: date_time_now
                .replace_date(in_date)
                .replace_time(Time::MIDNIGHT), // date: in_date,
                                               // time: Time::MIDNIGHT,
        }
    } else {
        DateTimeKeeper {
            initial_utc: date_time_now,
            // date: date_time_now.date(),
            // time: date_time_now.time(),
        }
    }
}

impl Sub for DateTimeKeeper {
    type Output = Duration;

    fn sub(self, other: Self) -> Self::Output {
        self.initial_utc - other.initial_utc
    }
}
