use crate::dateinfo::common;
use anyhow::Result;
use lifestuff_types::dateinfo::diff::{DateDuration, Diff};

fn do_output_format(breakdown: i64, duration: &str) -> String {
    let duration_display = if breakdown == 1 {
        duration.strip_suffix("s").unwrap()
    } else {
        duration
    };

    format!("{breakdown} full {duration_display}")
}

pub fn do_diff_date(diff_args: &Diff, verbose: bool) -> Result<()> {
    let first_date = common::get_date_from_string_arg(Some(&diff_args.date1), verbose)?;
    let second_date = common::get_date_from_string_arg(diff_args.date2.as_deref(), verbose)?;
    if verbose {
        println!(
            "Doing a date diff with {:?} and {:?}",
            first_date, second_date
        );
    }
    let date_diff = (first_date - second_date).abs();
    for option in &diff_args.to {
        let duration_type = option.to_string();
        let output = match option {
            DateDuration::Days => do_output_format(date_diff.whole_days(), &duration_type),
            DateDuration::Hours => do_output_format(date_diff.whole_hours(), &duration_type),
            DateDuration::Weeks => do_output_format(date_diff.whole_weeks(), &duration_type),
            DateDuration::Years => do_output_format(date_diff.whole_days() / 365, &duration_type),
        };

        println!("{output}");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use lifestuff_types::dateinfo::diff::{DateDuration, Diff};

    #[test]
    fn test_do_output_format_singular() {
        let result = do_output_format(1, "days");
        assert_eq!(result, "1 full day");
    }

    #[test]
    fn test_do_output_format_plural() {
        let result = do_output_format(5, "days");
        assert_eq!(result, "5 full days");
    }

    #[test]
    fn test_do_diff_date_days() {
        let diff_args = Diff {
            date1: "05/01/2023".to_string(),
            date2: Some("01/01/2023".to_string()),
            to: vec![DateDuration::Days],
        };
        let result = do_diff_date(&diff_args, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_do_diff_date_hours() {
        let diff_args = Diff {
            date1: "01/01/2023".to_string(),
            date2: Some("02/01/2023".to_string()),
            to: vec![DateDuration::Hours],
        };
        let result = do_diff_date(&diff_args, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_do_diff_date_weeks() {
        let diff_args = Diff {
            date1: "15/01/2023".to_string(),
            date2: Some("01/01/2023".to_string()),
            to: vec![DateDuration::Weeks],
        };
        let result = do_diff_date(&diff_args, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_do_diff_date_years() {
        let diff_args = Diff {
            date1: "01/01/2024".to_string(),
            date2: Some("01/01/2023".to_string()),
            to: vec![DateDuration::Years],
        };
        let result = do_diff_date(&diff_args, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_do_diff_date_multiple_durations() {
        let diff_args = Diff {
            date1: "05/01/2023".to_string(),
            date2: Some("01/01/2023".to_string()),
            to: vec![DateDuration::Days, DateDuration::Hours, DateDuration::Weeks],
        };
        let result = do_diff_date(&diff_args, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_do_diff_date_verbose() {
        let diff_args = Diff {
            date1: "05/01/2023".to_string(),
            date2: Some("01/01/2023".to_string()),
            to: vec![DateDuration::Days],
        };
        let result = do_diff_date(&diff_args, true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_do_diff_date_no_second_date() {
        let diff_args = Diff {
            date1: "05/01/2023".to_string(),
            date2: None,
            to: vec![DateDuration::Days],
        };
        let result = do_diff_date(&diff_args, false);
        assert!(result.is_ok());
    }
}
