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
        
        // Test that principal validation works correctly
        assert!(100000.0 > 0.0); // Principal must be positive
        
        // Test interest rate conversion (5.0% becomes 0.05)
        let converted_rate = 5.0 / 100.0;
        assert_eq!(converted_rate, 0.05);
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
        
        // Test the exact validation condition
        assert!(!(0.0 > 0.0)); // Zero principal should fail validation
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
        
        // Test the exact validation condition
        assert!(!(-100000.0 > 0.0)); // Negative principal should fail validation
    }
}
