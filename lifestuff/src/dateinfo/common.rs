use crate::dateinfo::DateTimeKeeper;
use anyhow::Result;

pub fn get_date_from_string_arg(input_date: Option<&str>, verbose: bool) -> Result<DateTimeKeeper> {
    if input_date.is_some() {
        let input_date_str = input_date.unwrap();
        if input_date_str.contains(['/', '-']) {
            DateTimeKeeper::new_from_dmy_str(input_date_str, verbose)
        } else {
            DateTimeKeeper::new_from_yyyymmdd_str(input_date_str, verbose)
        }
    } else {
        Ok(DateTimeKeeper::new_at_midnight())
    }
}
