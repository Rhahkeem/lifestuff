use crate::dateinfo::DateTimeKeeper;
use anyhow::Result;

pub fn handle_ordinal_operations() -> Result<()> {
    let today = DateTimeKeeper::now();
    println!("Today is {:?}", today);
    println!("{} days passed in the year", today.days_passed_in_year());
    println!("{} days remaining in the year", today.days_left_in_year());
    Ok(())
}
