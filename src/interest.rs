use crate::dateinfo::{get_date_from_string_arg, DateTimeKeeper};
use anyhow::{Ok, Result};
use clap::Args;
use time::{util, Date, Month};

#[derive(Debug, Args, Clone)]
pub struct Interest {
    #[clap(
        help = "Principal left on mortgage",
        short,
        long,
        allow_negative_numbers = false,
        required = true
    )]
    principal: f32,
    #[clap(
        help = "Interest rate (%)",
        short,
        long,
        allow_negative_numbers = false,
        required = true
    )]
    interest_rate: f32,
    #[clap(
        help = "Monthly payment amount",
        long,
        allow_negative_numbers = false,
        required = true
    )]
    repayment: f32,
    #[clap(
        help = "Max annual repayment percentage (%)",
        short,
        long,
        allow_negative_numbers = false,
        required = true,
        visible_alias = "annual-limit"
    )]
    max_repayment_pct: Option<u8>,
    #[clap(
        help = "Max annual supplemntary downpayment ",
        short,
        conflicts_with = "max_repayment_pct",
        long,
        required = true,
        allow_negative_numbers = false,
        visible_alias = "annual-downpayment"
    )]
    annual_downpayment: Option<f32>,
    #[clap(
        help = "Mortgage calculation end date (dd/mm/yyyy)",
        short,
        long,
        allow_negative_numbers = false
    )]
    end_date: String,
}
fn get_start_of_next_month(verbose: bool) -> Result<DateTimeKeeper> {
    let target_date = get_date_from_string_arg(None, verbose);
    let next_month = target_date.date().month().next();

    target_date
        .date()
        .replace_day(1)?
        .replace_month(next_month)?;
    Ok(target_date)
}

fn get_end_of_mortgage_period(end_date: &str, verbose: bool) -> Result<DateTimeKeeper> {
    let target_date = get_date_from_string_arg(Some(end_date), verbose);
    let num_days_in_month =
        util::days_in_year_month(target_date.date().year(), target_date.date().month());

    target_date.date().replace_day(num_days_in_month)?;

    Ok(target_date)
}

fn is_first_of_month(date: &Date) -> bool {
    date.day() == 1
}

fn is_beginning_of_year(date: &Date) -> bool {
    date.month() == Month::January
}
pub fn handle_interest_calculations(interest_args: &Interest, verbose: bool) -> Result<()> {
    if verbose {
        println!("Interest Args: {:?}", interest_args);
    }
    let mortgage_end_date = get_end_of_mortgage_period(&interest_args.end_date, verbose)?;

    let mortgage_start_date = get_start_of_next_month(verbose)?;

    calculate_interest_data_for_period(
        &mortgage_start_date,
        &mortgage_end_date,
        interest_args.repayment,
        interest_args.interest_rate / 100 as f32,
        interest_args.principal,
        interest_args.max_repayment_pct,
        interest_args.annual_downpayment,
        verbose,
    );

    Ok(())
}

fn calculate_interest_data_for_period(
    start_date: &DateTimeKeeper,
    end_date: &DateTimeKeeper,
    monthly_payment: f32,
    interest_rate: f32,
    mut principal: f32,
    max_annual_repayment_pct: Option<u8>,
    annual_downpayment: Option<f32>,
    verbose: bool,
) {
    let mut current_date = start_date.date();
    let tota_num_days = (*end_date - *start_date).abs().whole_days();
    let mut accrued_monthly_interest = 0.0;
    let mut is_leap_year = time::util::is_leap_year(current_date.year());
    let mut total_paid = 0.0;
    let original_principal = principal;

    let using_interest = annual_downpayment.is_none();

    for _ in 1..=tota_num_days {
        match is_first_of_month(&current_date) {
            true => {
                if verbose {
                    println!(
                        "Beginning of the month: {:?}. Accrued interest last month was {:.2}",
                        current_date.to_calendar_date(),
                        accrued_monthly_interest
                    );
                }
                accrued_monthly_interest = 0.0;
                if is_beginning_of_year(&current_date) {
                    is_leap_year = time::util::is_leap_year(current_date.year());
                    if verbose {
                        println!(
                            "Beginning of the year: {:?} ",
                            current_date.to_calendar_date()
                        );
                    }

                    let repayment = if using_interest {
                        let interest_repayment = max_annual_repayment_pct.unwrap() as f32 / 100.0;

                        principal * interest_repayment
                    } else {
                        annual_downpayment.unwrap()
                    };
                    total_paid += repayment;
                    principal -= repayment;
                    if verbose {
                        println!("The max repayment number is {repayment}");
                        println!("After yearly payment the principal is now {principal}");
                    }
                }
                total_paid += monthly_payment;
                principal -= monthly_payment;
            }
            false => {
                let daily_interest =
                    interest_rate * principal / (if is_leap_year { 366.0 } else { 365.0 });
                accrued_monthly_interest += daily_interest;
                principal += daily_interest;
            }
        }

        current_date = current_date.next_day().unwrap();
    }

    println!(
        "Current date is {:?}, Principal is now : {principal}, Total paid is {total_paid}. {:.2} paid in interest",
        current_date, (total_paid- (original_principal - principal))
    );
}
