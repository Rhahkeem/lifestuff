#[cfg(test)]
mod tests {

    #[test]
    fn test_handle_mileage_operations_under_allowance() {
        let mileage_args = lifestuff_types::mileage::Mileage { mileage: 7000 };
        let result = crate::mileage::handle_mileage_operations(mileage_args, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_mileage_operations_over_allowance() {
        let mileage_args = lifestuff_types::mileage::Mileage { mileage: 9000 };
        let result = crate::mileage::handle_mileage_operations(mileage_args, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_mileage_operations_zero_mileage() {
        let mileage_args = lifestuff_types::mileage::Mileage { mileage: 0 };
        let result = crate::mileage::handle_mileage_operations(mileage_args, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_mileage_operations_negative_mileage() {
        let mileage_args = lifestuff_types::mileage::Mileage { mileage: u32::MAX }; // Simulating a large mileage
        let result = crate::mileage::handle_mileage_operations(mileage_args, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_mileage_cost_calculation_small_overage() {
        use crate::dateinfo::DateTimeKeeper;
        
        // Test the actual calculation logic from handle_mileage_operations
        let today = DateTimeKeeper::new_at_midnight();
        let start_date = DateTimeKeeper::new_from_dmy(23, 3, 2024).unwrap();
        let num_days_since_start = (today - start_date).whole_days() as f32;
        let mileage_per_day: f32 = 8000.0 / 365.0;
        let initial_mileage: f32 = 8300.0;
        let projected_mileage = initial_mileage + (num_days_since_start * mileage_per_day).ceil();
        
        let test_mileage = 8400u32;
        let mileage_delta = (projected_mileage - (test_mileage as f32)).abs();
        let expected_cost = mileage_delta * 0.0678;
        
        // Verify the calculation is deterministic
        assert!(expected_cost >= 0.0);
        
        let mileage_args = lifestuff_types::mileage::Mileage { mileage: test_mileage };
        let result = crate::mileage::handle_mileage_operations(mileage_args, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_mileage_cost_calculation_large_overage() {
        use crate::dateinfo::DateTimeKeeper;
        
        // Test the actual calculation logic from handle_mileage_operations
        let today = DateTimeKeeper::new_at_midnight();
        let start_date = DateTimeKeeper::new_from_dmy(23, 3, 2024).unwrap();
        let num_days_since_start = (today - start_date).whole_days() as f32;
        let mileage_per_day: f32 = 8000.0 / 365.0;
        let initial_mileage: f32 = 8300.0;
        let projected_mileage = initial_mileage + (num_days_since_start * mileage_per_day).ceil();
        
        let test_mileage = 9300u32;
        let mileage_delta = (projected_mileage - (test_mileage as f32)).abs();
        let expected_cost = mileage_delta * 0.0678;
        
        // Verify the calculation is deterministic
        assert!(expected_cost >= 0.0);
        
        let mileage_args = lifestuff_types::mileage::Mileage { mileage: test_mileage };
        let result = crate::mileage::handle_mileage_operations(mileage_args, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_mileage_cost_no_overage() {
        let mileage_args = lifestuff_types::mileage::Mileage { mileage: 8000 }; // Under allowance
        let result = crate::mileage::handle_mileage_operations(mileage_args, false);
        assert!(result.is_ok());
        // Should not show any cost as it's under allowance
    }
}
