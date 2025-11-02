#[cfg(test)]
mod tests {

    #[test]
    fn test_handle_interest_calculations_valid() {
        // Test that the interest calculation workflow completes successfully
        // with valid input parameters
        let interest_args = lifestuff_types::interest::Interest {
            principal: 100000.0,
            interest_rate: 5.0,
            repayment: 2000.0,
            max_repayment_pct: Some(10),
            annual_downpayment: Some(5000.0),
            end_date: "31/12/2025".to_string(),
        };

        // Test the actual calculation logic runs without error
        let result = crate::interest::handle_interest_calculations(interest_args.clone(), false);
        assert!(result.is_ok());

        // Test that different parameter combinations work correctly
        let interest_args_no_downpayment = lifestuff_types::interest::Interest {
            principal: 50000.0,
            interest_rate: 3.5,
            repayment: 1500.0,
            max_repayment_pct: Some(5),
            annual_downpayment: None, // Testing different branch of logic
            end_date: "30/06/2024".to_string(),
        };

        let result2 =
            crate::interest::handle_interest_calculations(interest_args_no_downpayment, false);
        assert!(result2.is_ok());
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
        // Test that the validation logic properly rejects zero principal
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
        // Verify the error message contains expected validation text
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("positive principal"));
    }

    #[test]
    fn test_handle_interest_calculations_negative_principal() {
        // Test that the validation logic properly rejects negative principal
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
        // Verify the error message contains expected validation text
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("positive principal"));
    }
}
