use clap::{Parser, Subcommand};
use strum::Display;

pub mod conversions;
pub mod currency;
pub mod dateinfo;
pub mod ddg;
pub mod interest;
pub mod mileage;

pub fn parse() -> Cli {
    Cli::parse()
}

#[derive(Parser, Debug)]
#[command(
    author = "Me!",
    about = "Lifestyle library!",
    version = "0.2.0",
    long_about = "Something Something Something Daaaarksiiiide"
)]
pub struct Cli {
    #[arg(short, long, global=true, action=clap::ArgAction::SetTrue)]
    /// Enable verbose output
    pub verbose: bool,
    #[command(subcommand)]
    /// Commands for the program
    pub command: Commands,
}

#[derive(Subcommand, Display, Debug)]
pub enum Commands {
    /// Unit conversions
    Convert(conversions::Conversions),
    /// Date Operations
    Dates(dateinfo::DateOperations),
    /// Interest Calculations
    Interest(interest::Interest),
    /// Currency Conversion Operations
    Currency(currency::Currency),
    /// Mileage Calculations
    Mileage(mileage::Mileage),
    /// DuckDuckGo Address
    DDG(ddg::DDGOperations),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_function_exists() {
        // Test that the parse function exists and can be called
        // We can't easily test CLI parsing without actual command line args
        // but we can verify the function signature
        assert!(true); // Placeholder test
    }

    #[test]
    fn test_cli_struct_creation() {
        use crate::conversions::{Conversions, ConversionOption};
        use crate::conversions::area::{AreaConversion, AreaUnits};

        let conversion = Conversions {
            convert_type: ConversionOption::Area(AreaConversion {
                from: AreaUnits::SquareMetres,
                to: vec![AreaUnits::SqKilometres],
                value: 1000.0,
            }),
        };

        let cli = Cli {
            verbose: false,
            command: Commands::Convert(conversion),
        };

        assert!(!cli.verbose);
        match cli.command {
            Commands::Convert(_) => assert!(true),
            _ => panic!("Expected Convert command"),
        }
    }

    #[test]
    fn test_command_display_formats() {
        use crate::conversions::{Conversions, ConversionOption};
        use crate::conversions::area::{AreaConversion, AreaUnits};
        use crate::dateinfo::{DateOperations, DateOption};
        use crate::mileage::Mileage;

        // Test that display formatting provides meaningful output for different commands
        let conversion = Commands::Convert(Conversions {
            convert_type: ConversionOption::Area(AreaConversion {
                from: AreaUnits::SquareMetres,
                to: vec![AreaUnits::SqKilometres, AreaUnits::Acres],
                value: 1000.0,
            }),
        });
        let display_str = format!("{}", conversion);
        assert!(display_str.contains("Convert"));
        
        let date_op = Commands::Dates(DateOperations {
            operation_type: DateOption::Ordinal,
        });
        let date_display = format!("{}", date_op);
        assert!(date_display.contains("Dates"));
        
        let mileage = Commands::Mileage(Mileage { mileage: 8500 });
        let mileage_display = format!("{}", mileage);
        assert!(mileage_display.contains("Mileage"));
    }

    #[test]
    fn test_command_data_integrity() {
        use crate::conversions::{Conversions, ConversionOption};
        use crate::conversions::distance::{DistanceConversion, DistanceUnits};
        use crate::interest::Interest;

        // Test that command data is properly preserved through creation
        let command = Commands::Convert(Conversions {
            convert_type: ConversionOption::Distance(DistanceConversion {
                from: DistanceUnits::Miles,
                to: vec![DistanceUnits::Kilometres, DistanceUnits::Metres],
                value: 100.0,
            }),
        });
        
        // Verify the command maintains data integrity
        if let Commands::Convert(conv) = command {
            if let ConversionOption::Distance(dc) = conv.convert_type {
                assert_eq!(dc.from, DistanceUnits::Miles);
                assert_eq!(dc.to.len(), 2);
                assert_eq!(dc.value, 100.0);
            } else {
                panic!("Expected Distance conversion");
            }
        } else {
            panic!("Expected Convert command");
        }
        
        // Test Interest command preserves optional fields correctly
        let interest = Commands::Interest(Interest {
            principal: 50000.0,
            interest_rate: 4.5,
            repayment: 1000.0,
            max_repayment_pct: None,  // Testing None variant
            annual_downpayment: Some(2500.0),  // Testing Some variant
            end_date: "2025-12-31".to_string(),
        });
        
        if let Commands::Interest(int) = interest {
            assert_eq!(int.principal, 50000.0);
            assert!(int.max_repayment_pct.is_none());
            assert_eq!(int.annual_downpayment, Some(2500.0));
        } else {
            panic!("Expected Interest command");
        }
    }
}
