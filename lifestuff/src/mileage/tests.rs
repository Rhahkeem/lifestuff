
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_mileage_operations_under_allowance() {
        let mileage_args = Mileage { mileage: 7000 };
        let result = handle_mileage_operations(&mileage_args, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_mileage_operations_over_allowance() {
        let mileage_args = Mileage { mileage: 9000 };
        let result = handle_mileage_operations(&mileage_args, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_mileage_operations_zero_mileage() {
        let mileage_args = Mileage { mileage: 0 };
        let result = handle_mileage_operations(&mileage_args, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_mileage_operations_negative_mileage() {
        let mileage_args = Mileage { mileage: u32::MAX }; // Simulating a large mileage
        let result = handle_mileage_operations(&mileage_args, false);
        assert!(result.is_ok());
    }
}
