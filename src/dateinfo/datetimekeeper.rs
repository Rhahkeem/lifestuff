use anyhow::{ensure, Context, Error, Result};
use std::fmt;
use std::ops::{Add, AddAssign, Sub};
use time::macros::format_description;
use time::util::days_in_year;
use time::{Date, Duration, Month, OffsetDateTime, PrimitiveDateTime, Time};

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
        yyyymmdd > 10000000 && yyyymmdd < 100000000,
        "Error parsing `yyyymmdd` date format. Invalid number of digits found:{}",
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

    let (tokens, year_idx, month_idx, date_idx) = (
        input
            .split(['/', '-'])
            .map(|token| token.trim())
            .collect::<Vec<&str>>(),
        2,
        1,
        0,
    );

    if verbose {
        println!("The tokens were {:?}", tokens);
    }

    ensure!(
        tokens.len() == 3,
        "Error handling date parsing. Found less than 3 tokens to parse"
    );

    let year = u32::from_str_radix(tokens[year_idx], 10).context(format!(
        "Error handling year parsing. Could not convert '{}' to a number",
        tokens[year_idx]
    ))?;

    if verbose {
        println!("Year is {:?}", year);
    }

    let month = u8::from_str_radix(tokens[month_idx], 10).context(format!(
        "Error handling month parsing. Could not convert '{}' to a valid number",
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

fn get_last_day_of_proposed_month(year: i32, month: time::Month) -> u8 {
    time::util::days_in_year_month(year, month)
}

#[doc = r"Constructors"]
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

    pub fn new_from_dmy(day: u8, month: u8, year: u32) -> Result<Self> {
        let the_date = format!("{:04}-{:02}-{:02} 00:00:00", year, month, day);
        let date_format = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
        let date_time = PrimitiveDateTime::parse(&the_date, &date_format)?.assume_utc();

        Ok(Self {
            utc_date_time: date_time,
        })
    }

    pub fn new_from_dmy_str(input: &str, verbose: bool) -> Result<Self> {
        let parsed_date = parse_input_date_dmy(input, verbose)?;

        Self::new_from_dmy(parsed_date.2, parsed_date.1, parsed_date.0)
    }

    pub fn new_from_yyyymmdd_str(input: &str, verbose: bool) -> Result<Self> {
        let parsed_date = parse_input_date_yyyymmdd(input, verbose)?;
        Self::new_from_dmy(parsed_date.2, parsed_date.1, parsed_date.0)
    }
}

#[doc = r"Accessors"]
impl DateTimeKeeper {
    pub fn date(&self) -> time::Date {
        self.utc_date_time.date()
    }

    pub fn time(&self) -> Time {
        self.utc_date_time.time()
    }
}

#[doc = r"Manipulators"]
impl DateTimeKeeper {
    #[allow(dead_code)]
    pub fn set_date(&mut self, date: &time::Date) {
        self.utc_date_time = self.utc_date_time.replace_date(*date);
    }

    #[allow(dead_code)]
    pub fn set_time(&mut self, time: &time::Time) {
        self.utc_date_time = self.utc_date_time.replace_time(*time);
    }

    #[allow(dead_code)]
    pub fn set_date_ymd(&mut self, year: i32, month: &time::Month, day: u8) -> Result<()> {
        let date = Date::from_calendar_date(year, *month, day)?;
        self.set_date(&date);

        Ok(())
    }

    #[allow(dead_code)]
    pub fn set_year(&mut self, year: i32) -> Result<()> {
        ensure!(
            year > 0,
            "Attempted to set year to a negative value. Hint: Use apply year delta instead"
        );

        let interim = self.utc_date_time.replace_year(year);

        ensure!(
            interim.is_ok(),
            "Unable to update year. Err: {:?}",
            interim.err()
        );

        Ok(self.utc_date_time = interim.unwrap())
    }
    #[allow(dead_code)]
    pub fn set_month(&mut self, month: time::Month) -> Result<()> {
        let interim = self.utc_date_time.replace_month(month);

        ensure!(
            interim.is_ok(),
            "Unable to update month. Err: {:?}",
            interim.err()
        );

        Ok(self.utc_date_time = interim.unwrap())
    }
    #[allow(dead_code)]
    pub fn set_month_num(&mut self, month_num: u8) -> Result<()> {
        ensure!(
            month_num >= 1 && month_num <= 12,
            "Invalid month passed. Must be between 1 and 12"
        );

        let month = Month::try_from(month_num).unwrap();

        self.set_month(month)
    }
    #[allow(dead_code)]
    pub fn set_day(&mut self, day: u8) -> Result<()> {
        let interim = self.utc_date_time.replace_day(day);

        ensure!(
            interim.is_ok(),
            "Unable to set day. Err: {:?}",
            interim.err()
        );

        Ok(self.utc_date_time = interim.unwrap())
    }

    #[allow(dead_code)]
    pub fn set_time_hms(
        &mut self,
        hour: Option<u8>,
        minute: Option<u8>,
        second: Option<u8>,
    ) -> Result<()> {
        ensure!(hour.is_some() || minute.is_some() || second.is_some(), "Invalid options passed for update time. Specify at least one of hour, minute or second");

        let interim = self
            .utc_date_time
            .replace_hour(hour.unwrap_or_default())?
            .replace_minute(minute.unwrap_or_default())?
            .replace_second(second.unwrap_or_default())?;

        Ok(self.utc_date_time = interim)
    }

    pub fn apply_year_delta(&self, years_delta: i32) -> Result<Self> {
        if years_delta.is_positive() {
            ensure!(
                self.utc_date_time.date() != Date::MAX,
                "Cannot increment year on Max Date"
            );
        } else if years_delta.is_negative() {
            ensure!(
                self.utc_date_time.date() != Date::MIN,
                "Cannot decrement year on Min Date"
            );
        } else {
            return Ok(*self);
        }

        let current_day = self.date().day();
        let current_year = self.date().year();
        let current_month = self.date().month();

        let is_last_day_of_month = if current_day >= 28 {
            self.is_last_day_of_month()
        } else {
            false
        };

        let result_date = if is_last_day_of_month {
            self.utc_date_time.replace_date(Date::from_calendar_date(
                current_year + years_delta,
                current_month,
                get_last_day_of_proposed_month(current_year + years_delta, current_month),
            )?)
        } else {
            self.utc_date_time
                .replace_year(self.utc_date_time.year() + years_delta)?
        };

        Ok(Self {
            utc_date_time: result_date,
        })
    }

    pub fn apply_month_delta(&self, months_delta: i32) -> Result<Self> {
        if months_delta.is_positive() {
            ensure!(
                self.utc_date_time.date() != Date::MAX,
                "Cannot increment month on Max Date"
            );
        } else {
            ensure!(
                self.utc_date_time.date() != Date::MIN,
                "Cannot decrement month on Min Date"
            );
        }

        let current_day = self.date().day();
        let current_year = self.date().year();

        let is_currently_last_day_of_month = if current_day >= 28 {
            self.is_last_day_of_month()
        } else {
            false
        };

        let mut whole_years = months_delta / 12;
        let leftovers = (months_delta.abs() % 12) as u8;

        let current_month = u8::from(self.utc_date_time.month());

        let new_month_idx = if months_delta.is_positive() {
            if current_month + leftovers > 12 {
                whole_years += 1;
                leftovers - (12 - current_month)
            } else {
                current_month + leftovers
            }
        } else {
            if (current_month as i32 - leftovers as i32) < 1 {
                whole_years -= 1;
                12 - (leftovers - current_month)
            } else {
                current_month - leftovers
            }
        };

        let last_day_of_proposed_month =
            get_last_day_of_proposed_month(current_year, time::Month::try_from(new_month_idx)?);

        let new_instance =
            if is_currently_last_day_of_month &&   current_day > last_day_of_proposed_month {
                self.apply_year_delta(whole_years)?
                    .utc_date_time
                    .replace_day(last_day_of_proposed_month)?
                    .replace_month(time::Month::try_from(new_month_idx)?)?
                    
            } else {
                self.apply_year_delta(whole_years)?
                    .utc_date_time
                    .replace_month(time::Month::try_from(new_month_idx)?)?
            };

        Ok(Self {
            utc_date_time: new_instance,
        })
    }

    /// Get the beginning of the next year
    #[allow(dead_code)]
    pub fn next_year(&self) -> Result<Self> {
        ensure!(
            self.utc_date_time.date() != Date::MAX,
            "Cannot increment year on Max Date"
        );

        let next_year = self.utc_date_time.year() + 1;
        let new_date = Date::from_calendar_date(next_year, Month::January, 1)?;
        Ok(Self {
            utc_date_time: self.utc_date_time.replace_date(new_date),
        })
    }

    ///Get the beginning of the next month
    #[allow(dead_code)]
    pub fn next_month(&self) -> Result<Self> {
        ensure!(
            self.utc_date_time.date() != Date::MAX,
            "Cannot increment Month on Max Date"
        );

        let next_month = self.utc_date_time.month().next();
        let year = if next_month == Month::January {
            self.utc_date_time.date().year() + 1
        } else {
            self.utc_date_time.date().year()
        };
        let new_date = Date::from_calendar_date(year, next_month, 1)?;
        Ok(Self {
            utc_date_time: self.utc_date_time.replace_date(new_date),
        })
    }
}

#[doc = r"Utility Functions"]
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

impl DateTimeKeeper {
    pub fn is_last_day_of_month(&self) -> bool {
        let current_date = self.date();
        current_date.day()
            == time::util::days_in_year_month(current_date.year(), current_date.month())
    }

    pub fn days_left_in_year(&self) -> u16 {
        days_in_year(self.date().year()) - self.date().ordinal()
    }

    pub fn days_passed_in_year(&self) -> u16 {
        self.date().ordinal() - 1
    }

    pub fn ordinal_week(&self) -> u8 {
        self.date().iso_week()
    }
}

#[cfg(test)]
mod tests;
