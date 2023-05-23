use crate::dateinfo::DateTimeKeeper;

pub fn get_date_from_string_arg(input_date: Option<&str>, verbose: bool) -> DateTimeKeeper {
    if input_date.is_some() {
        let input_date_str = input_date.unwrap();
        if input_date_str.contains(['/', '-']) {
            DateTimeKeeper::new_from_dmy_str(input_date_str, verbose)
        } else {
            DateTimeKeeper::new_from_yyyymmdd_str(input_date_str, verbose)
        }
    } else {
        DateTimeKeeper::new_at_midnight()
    }
}
