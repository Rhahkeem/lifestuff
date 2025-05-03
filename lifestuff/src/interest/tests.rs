#[cfg(test)]
mod tests {

    #[test]
    fn test_handle_interest_calculations_valid() {
        let interest_args = lifestuff_types::interest::Interest {
            principal: 100000.0,
            interest_rate: 5.0,
            repayment: 2000.0,
            max_repayment_pct: Some(10),
            annual_downpayment: Some(5000.0),
            end_date: "31/12/2025".to_string(),
        };
        let result = crate::interest::handle_interest_calculations(interest_args, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_interest_calculations_invalid_end_date() {
        let interest_args = lifestuff_types::interest::Interest {
            principal: 100000.0,
            interest_rate: 5.0,
            repayment: 2000.0,
            max_repayment_pct: Some(10),
            annual_downpayment: Some(5000.0),
            end_date: "invalid_date".to_string(),
        };
        let result = crate::interest::handle_interest_calculations(interest_args, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_handle_interest_calculations_zero_principal() {
        let interest_args = lifestuff_types::interest::Interest {
            principal: 0.0,
            interest_rate: 5.0,
            repayment: 2000.0,
            max_repayment_pct: Some(10),
            annual_downpayment: Some(5000.0),
            end_date: "31/12/2025".to_string(),
        };
        let result = crate::interest::handle_interest_calculations(interest_args, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_handle_interest_calculations_negative_principal() {
        let interest_args = lifestuff_types::interest::Interest {
            principal: -100000.0,
            interest_rate: 5.0,
            repayment: 2000.0,
            max_repayment_pct: Some(10),
            annual_downpayment: Some(5000.0),
            end_date: "31/12/2025".to_string(),
        };
        let result = crate::interest::handle_interest_calculations(interest_args, false);
        assert!(result.is_err());
    }
}
