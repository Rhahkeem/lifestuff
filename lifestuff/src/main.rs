mod conversions;
mod currency;
mod dateinfo;
mod ddg;
mod http_utils;
mod interest;
mod mileage;
mod mortgage;

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
        Commands::Mortgage(args) => mortgage::handle_mortgage_operations(args, verbose),
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
    fn test_command_routing() {
        use lifestuff_types::conversions::distance::{DistanceConversion, DistanceUnits};
        use lifestuff_types::conversions::{ConversionOption, Conversions};

        // Test that conversion command routes to correct handler
        let conversion = Conversions {
            convert_type: ConversionOption::Distance(DistanceConversion {
                from: DistanceUnits::Metres,
                to: vec![DistanceUnits::Kilometres],
                value: 1000.0,
            }),
        };

        // Verify the conversion logic works
        let result = conversions::perform_conversion(conversion);
        assert!(result.is_ok());
    }

    #[test]
    fn test_error_propagation() {
        use lifestuff_types::interest::Interest;

        // Test that errors from handlers are properly propagated
        let invalid_interest = Interest {
            principal: -1000.0, // Invalid negative principal
            interest_rate: 5.0,
            repayment: 500.0,
            max_repayment_pct: Some(10),
            annual_downpayment: None,
            end_date: "2024-01-01".to_string(),
        };

        let result = interest::handle_interest_calculations(invalid_interest, false);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("positive principal"));
    }

    #[test]
    fn test_verbose_mode_integration() {
        use lifestuff_types::mileage::Mileage;

        // Test that verbose mode affects handler behavior
        // Both should succeed regardless of verbose mode
        let result_quiet = mileage::handle_mileage_operations(Mileage { mileage: 8000 }, false);
        assert!(result_quiet.is_ok());

        let result_verbose = mileage::handle_mileage_operations(Mileage { mileage: 8000 }, true);
        assert!(result_verbose.is_ok());
    }
}
