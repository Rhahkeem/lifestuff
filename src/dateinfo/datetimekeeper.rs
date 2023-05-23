use anyhow::{ensure, Context, Error, Result};
use std::fmt;
use std::ops::{Add, AddAssign, Sub};
use time::macros::format_description;
use time::{Duration, OffsetDateTime, PrimitiveDateTime, Time};

#[derive(Clone, Copy)]
pub struct DateTimeKeeper {
    utc_date_time: OffsetDateTime,
}

#[doc = r"Helper Functions"]
fn parse_input_date_yyyymmdd(input: &str, verbose: bool) -> Result<(u32, u8, u8), Error> {
    let yyyymmdd = u32::from_str_radix(input, 10)
        .context(format!("Unable to parse '{input}' into a valid number"))?;
    if verbose {
        println!("String parsed date is: {yyyymmdd}");
    }
    ensure!(
        yyyymmdd > 10000000 && yyyymmdd < 10000000,
        "Error parsing `yyyymmdd` date format. Invalid number of digits found:{}.",
        yyyymmdd.to_string().len()
    );
    return Ok((
        yyyymmdd / 10000,
        ((yyyymmdd / 100) % 100) as u8,
        (yyyymmdd % 100) as u8,
    ));
}

fn parse_input_date_dmy(input: &str, verbose: bool) -> Result<(u32, u8, u8), Error> {
    ensure!(
        input.contains(['-','/']) && !(input.contains('/') && input.contains('-')) ,
        "Error handling date parsing. Expected delimiter format `-` or `/`. e.g dd/mm/yy or dd-mm-yyyy"
    );

    let (tokens, year_idx, month_idx, date_idx) =
        (input.split(['/', '-']).collect::<Vec<&str>>(), 2, 1, 0);

    if verbose {
        println!("The tokens were {:?}", tokens);
    }

    ensure!(
        tokens.len() == 3,
        "Error handling date parsing. Found less than 3 tokens to parse"
    );

    let year = u32::from_str_radix(tokens[year_idx], 10).context(format!(
        "Error handling year parsing. Couldn't convert '{}' to a number",
        tokens[year_idx]
    ))?;

    if verbose {
        println!("Year is {:?}", year);
    }

    let month = u8::from_str_radix(tokens[month_idx], 10).context(format!(
        "Error handling month parsing. Couldn't convert '{}' to a valid number",
        tokens[month_idx]
    ))?;

    ensure!(
        month > 0 && month <= 12,
        "Invalid Month passed {}. Must be between 0 and 12",
        month
    );

    if verbose {
        println!("Month is {:?}", month);
    }

    let date = u8::from_str_radix(tokens[date_idx], 10).context(format!(
        "Unable to convert {} to a valid number",
        tokens[date_idx]
    ))?;

    if verbose {
        println!("Date is {:?}", date);
    }
    return Ok((if year < 100 { year + 2000 } else { year }, month, date));
}

/// Constructors
impl DateTimeKeeper {
    #[allow(dead_code)]
    pub fn now() -> Self {
        let date_time_now = OffsetDateTime::now_utc();
        Self {
            utc_date_time: date_time_now,
        }
    }

    #[allow(dead_code)]
    pub fn new_at_midnight() -> Self {
        let date_time_now = OffsetDateTime::now_utc().replace_time(Time::MIDNIGHT);
        Self {
            utc_date_time: date_time_now,
        }
    }

    pub fn new_from_dmy(day: u8, month: u8, year: u32) -> Self {
        let the_date = format!("{:04}-{:02}-{:02} 00:00:00", year, month, day);
        let date_format = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
        let date_time = PrimitiveDateTime::parse(&the_date, &date_format)
            .unwrap()
            .assume_utc();

        Self {
            utc_date_time: date_time,
        }
    }

    pub fn new_from_dmy_str(input: &str, verbose: bool) -> Self {
        let parsed_date = parse_input_date_dmy(input, verbose);
        if parsed_date.is_err() {
            println!("Encountered error: {:?}", parsed_date.err());

            let date_time_now = OffsetDateTime::now_utc().replace_time(Time::MIDNIGHT);
            Self {
                utc_date_time: date_time_now,
            }
        } else {
            let date_vals = parsed_date.unwrap();
            Self::new_from_dmy(date_vals.2, date_vals.1, date_vals.0)
        }
    }

    pub fn new_from_yyyymmdd_str(input: &str, verbose: bool) -> Self {
        let parsed_date = parse_input_date_yyyymmdd(input, verbose);
        if parsed_date.is_err() {
            println!("Encountered error: {:?}", parsed_date.err());

            let date_time_now = OffsetDateTime::now_utc().replace_time(Time::MIDNIGHT);
            Self {
                utc_date_time: date_time_now,
            }
        } else {
            let date_vals = parsed_date.unwrap();
            Self::new_from_dmy(date_vals.2, date_vals.1, date_vals.0)
        }
    }
}

///Accessors
impl DateTimeKeeper {
    pub fn date(&self) -> time::Date {
        self.utc_date_time.date()
    }

    pub fn time(&self) -> time::Time {
        self.utc_date_time.time()
    }
}

/// Setters and Manipulators
impl DateTimeKeeper {
    #[allow(dead_code)]
    pub fn set_date(&mut self, date: &time::Date) {
        self.utc_date_time = self.utc_date_time.replace_date(*date);
    }

    #[allow(dead_code)]
    pub fn set_time(&mut self, time: &time::Time) {
        self.utc_date_time = self.utc_date_time.replace_time(*time);
    }

    pub fn update_years(&mut self, delta: i32) {
        self.utc_date_time = self
            .utc_date_time
            .replace_year(self.utc_date_time.year() + delta)
            .unwrap();
    }

    pub fn update_months(&mut self, delta: i32) {
        let mut whole_years = delta / 12;
        let leftovers = (delta % 12) as u8;

        let current_month = u8::from(self.utc_date_time.month());

        let month_idx = if current_month + leftovers > 12 {
            whole_years += 1;
            leftovers - (12 - current_month)
        } else {
            current_month + leftovers
        };

        self.update_years(whole_years);
        self.utc_date_time = self
            .utc_date_time
            .replace_month(time::Month::try_from(month_idx).unwrap())
            .unwrap();
    }

    pub fn update_days(&mut self, delta: i64) {
        self.utc_date_time = self.utc_date_time + Duration::days(delta);
    }

    pub fn update_weeks(&mut self, delta: i64) {
        self.utc_date_time = self.utc_date_time + Duration::weeks(delta);
    }

    #[allow(dead_code)]
    pub fn update_time(
        &self,
        hour: Option<u8>,
        minute: Option<u8>,
        second: Option<u8>,
    ) -> Result<()> {
        ensure!(hour.is_some() || minute.is_some() || second.is_some(), "Invalid options passsed for update time. Specify at least one of hour, minute or second");

        self.utc_date_time
            .replace_hour(hour.unwrap_or_default())?
            .replace_minute(minute.unwrap_or_default())?
            .replace_second(second.unwrap_or_default())?;

        Ok(())
    }
}

impl fmt::Debug for DateTimeKeeper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.utc_date_time.to_calendar_date())
    }
}

impl Sub for DateTimeKeeper {
    type Output = Duration;

    fn sub(self, other: Self) -> Self::Output {
        self.utc_date_time - other.utc_date_time
    }
}

impl Add<Duration> for DateTimeKeeper {
    type Output = Self;

    fn add(self, rhs: Duration) -> Self::Output {
        Self {
            utc_date_time: self.utc_date_time + rhs,
        }
    }
}

impl AddAssign<Duration> for DateTimeKeeper {
    fn add_assign(&mut self, rhs: Duration) {
        self.utc_date_time.add_assign(rhs);
    }
}
