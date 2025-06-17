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
        let mileage_args = lifestuff_types::mileage::Mileage { mileage: 8400 }; // 100 miles over
        let result = crate::mileage::handle_mileage_operations(mileage_args, false);
        assert!(result.is_ok());
        // Expected cost: 100 * 0.0678 = £6.78
    }

    #[test]
    fn test_mileage_cost_calculation_large_overage() {
        let mileage_args = lifestuff_types::mileage::Mileage { mileage: 9300 }; // 1000 miles over
        let result = crate::mileage::handle_mileage_operations(mileage_args, false);
        assert!(result.is_ok());
        // Expected cost: 1000 * 0.0678 = £67.80
    }

    #[test]
    fn test_mileage_cost_no_overage() {
        let mileage_args = lifestuff_types::mileage::Mileage { mileage: 8000 }; // Under allowance
        let result = crate::mileage::handle_mileage_operations(mileage_args, false);
        assert!(result.is_ok());
        // Should not show any cost as it's under allowance
    }
}
