mod tests;

use crate::dateinfo::DateTimeKeeper;
use anyhow::Result;
use colored::Colorize;
use lifestuff_types::mileage::Mileage;

pub fn handle_mileage_operations(mileage_args: Mileage, _: bool) -> Result<()> {
    let today = DateTimeKeeper::new_at_midnight();
    let start_date = DateTimeKeeper::new_from_dmy(23, 3, 2024)?;
    let num_days_since_start = (today - start_date).whole_days() as f32;
    let mileage_per_day: f32 = 8000.0 / 365.0;
    let initial_mileage: f32 = 8300.0;
    let projected_mileage = initial_mileage + (num_days_since_start * mileage_per_day).ceil();
    let under_allowance = (mileage_args.mileage as f32) < projected_mileage;
    let mileage_delta = (projected_mileage - (mileage_args.mileage as f32)).abs();
    let daily_delta = (mileage_delta / mileage_per_day).ceil().abs();
    let cost = mileage_delta * 0.0678;
    println!(
        "Current mileage is {}, projected mileage is {projected_mileage}. Current mileage is {} by {daily_delta} days or {mileage_delta} miles",
        mileage_args.mileage,
        if under_allowance {
            "under".italic().green()
        } else {
            "over".bold().red()
        }
    );
    if !under_allowance {
        println!("Cost of going over is £{cost:.2}");
    }
    Ok(())
}
