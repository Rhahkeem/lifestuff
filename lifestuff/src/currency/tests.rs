
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_currency_operations_valid() {
        let currency_args = Currency {
            from: "USD".to_string(),
            amt: 100.0,
            to: vec!["EUR".to_string()],
        };
        let result = handle_currency_operations(&currency_args, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_currency_operations_invalid_from_currency() {
        let currency_args = Currency {
            from: "INVALID".to_string(),
            amt: 100.0,
            to: vec!["EUR".to_string()],
        };
        let result = handle_currency_operations(&currency_args, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_handle_currency_operations_invalid_to_currency() {
        let currency_args = Currency {
            from: "USD".to_string(),
            amt: 100.0,
            to: vec!["INVALID".to_string()],
        };
        let result = handle_currency_operations(&currency_args, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_handle_currency_operations_zero_amount() {
        let currency_args = Currency {
            from: "USD".to_string(),
            amt: 0.0,
            to: vec!["EUR".to_string()],
        };
        let result = handle_currency_operations(&currency_args, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_currency_operations_negative_amount() {
        let currency_args = Currency {
            from: "USD".to_string(),
            amt: -100.0,
            to: vec!["EUR".to_string()],
        };
        let result = handle_currency_operations(&currency_args, false);
        assert!(result.is_ok());
    }
}
