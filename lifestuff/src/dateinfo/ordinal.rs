use crate::dateinfo::DateTimeKeeper;
use anyhow::Result;

pub fn handle_ordinal_operations() -> Result<()> {
    let today = DateTimeKeeper::now();
    println!("Today is {:?}", today);
    println!("{} days passed in the year", today.days_passed_in_year());
    println!("{} days remaining in the year", today.days_left_in_year());
    println!("This is week {} of the year ", today.ordinal_week());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_ordinal_operations() {
        let result = handle_ordinal_operations();
        assert!(result.is_ok());
    }
}
