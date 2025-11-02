mod add;
mod common;
mod datetimekeeper;
mod ordinal;
pub use common::*;
pub use datetimekeeper::*;
mod diff;
use anyhow::Result;
use lifestuff_types::dateinfo::{DateOperations, DateOption};

pub fn handle_date_operations(date_args: DateOperations, verbose: bool) -> Result<()> {
    match &date_args.operation_type {
        DateOption::Diff(diff_args) => diff::do_diff_date(diff_args, verbose),
        DateOption::Add(add_args) => add::do_add_date(add_args, verbose),
        DateOption::Ordinal => ordinal::handle_ordinal_operations(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lifestuff_types::dateinfo::add::{Add, TimePeriod};
    use lifestuff_types::dateinfo::diff::{DateDuration, Diff};
    use lifestuff_types::dateinfo::{DateOperations, DateOption};

    #[test]
    fn test_handle_date_operations_diff() {
        let diff_args = Diff {
            date1: "01/01/2023".to_string(),
            date2: Some("02/01/2023".to_string()),
            to: vec![DateDuration::Days],
        };
        let date_ops = DateOperations {
            operation_type: DateOption::Diff(diff_args),
        };
        let result = handle_date_operations(date_ops, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_date_operations_add() {
        let add_args = Add {
            date: Some("01/01/2023".to_string()),
            val: 1,
            period: TimePeriod::Days,
        };
        let date_ops = DateOperations {
            operation_type: DateOption::Add(add_args),
        };
        let result = handle_date_operations(date_ops, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_date_operations_ordinal() {
        let date_ops = DateOperations {
            operation_type: DateOption::Ordinal,
        };
        let result = handle_date_operations(date_ops, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_date_operations_verbose() {
        let add_args = Add {
            date: Some("01/01/2023".to_string()),
            val: 1,
            period: TimePeriod::Days,
        };
        let date_ops = DateOperations {
            operation_type: DateOption::Add(add_args),
        };
        let result = handle_date_operations(date_ops, true);
        assert!(result.is_ok());
    }
}
