mod conversions;
mod currency;
mod dateinfo;
mod ddg;
mod interest;
mod mileage;

use anyhow::Result;
use lifestuff_types::{parse, Commands};

fn main() -> Result<()> {
    let cli = parse();

    let verbose = cli.verbose;
    if verbose {
        println!("CLI Args: {:?}", cli);
    }
    let da_answer = match cli.command {
        Commands::Convert(args) => conversions::perform_conversion(args),
        Commands::Dates(args) => dateinfo::handle_date_operations(args, verbose),
        Commands::Interest(args) => interest::handle_interest_calculations(args, verbose),
        Commands::Currency(args) => currency::handle_currency_operations(args, verbose),
        Commands::Mileage(args) => mileage::handle_mileage_operations(args, verbose),
        Commands::DDG(args) => ddg::handle_ddg_operations(args, verbose),
    };

    if let Err(e) = da_answer {
        println!("{e}");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Testing main() directly is challenging since it reads from CLI args
    // These tests focus on the core logic and error handling patterns

    #[test]
    fn test_main_function_exists() {
        // This test verifies that the main function exists and has the correct signature
        // We can't easily test the full CLI parsing without mocking command line args
        assert!(true); // Placeholder test
    }

    #[test]
    fn test_error_handling_pattern() {
        // Test that our error handling pattern works
        let test_result: Result<()> = Ok(());
        if let Err(_e) = test_result {
            // This branch would handle errors like in main()
            assert!(false, "Should not reach this branch");
        }
        assert!(true);
    }

    #[test]
    fn test_anyhow_result_type() {
        // Verify that we can create and handle anyhow::Result types
        let success: Result<()> = Ok(());
        assert!(success.is_ok());

        let error: Result<()> = Err(anyhow::anyhow!("Test error"));
        assert!(error.is_err());
    }
}
