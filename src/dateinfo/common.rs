use crate::dateinfo::DateTimeKeeper;
use anyhow::Result;

pub fn get_date_from_string_arg(input_date: Option<&str>, verbose: bool) -> Result<DateTimeKeeper> {
    if let Some(input_date) = input_date {
        if input_date.contains(['/', '-']) {
            DateTimeKeeper::new_from_dmy_str(input_date, verbose)
        } else {
            DateTimeKeeper::new_from_yyyymmdd_str(input_date, verbose)
        }
    } else {
        Ok(DateTimeKeeper::new_at_midnight())
    }
}
