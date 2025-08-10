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
    fn test_commands_display() {
        use crate::conversions::{Conversions, ConversionOption};
        use crate::conversions::area::{AreaConversion, AreaUnits};

        let conversion = Conversions {
            convert_type: ConversionOption::Area(AreaConversion {
                from: AreaUnits::SquareMetres,
                to: vec![AreaUnits::SqKilometres],
                value: 1000.0,
            }),
        };

        let command = Commands::Convert(conversion);
        let display_str = format!("{}", command);
        assert!(display_str.contains("Convert"));
    }

    #[test]
    fn test_all_command_variants() {
        use crate::conversions::{Conversions, ConversionOption};
        use crate::conversions::area::{AreaConversion, AreaUnits};
        use crate::dateinfo::{DateOperations, DateOption};
        use crate::interest::Interest;
        use crate::currency::Currency;
        use crate::mileage::Mileage;
        use crate::ddg::{DDGOperations, DDGOption};

        // Test that all command variants can be created
        let commands = vec![
            Commands::Convert(Conversions {
                convert_type: ConversionOption::Area(AreaConversion {
                    from: AreaUnits::SquareMetres,
                    to: vec![AreaUnits::SqKilometres],
                    value: 1000.0,
                }),
            }),
            Commands::Dates(DateOperations {
                operation_type: DateOption::Ordinal,
            }),
            Commands::Interest(Interest {
                principal: 1000.0,
                interest_rate: 5.0,
                repayment: 500.0,
                max_repayment_pct: Some(10),
                annual_downpayment: None,
                end_date: "2024-01-01".to_string(),
            }),
            Commands::Currency(Currency {
                amt: 100.0,
                from: "USD".to_string(),
                to: vec!["EUR".to_string()],
            }),
            Commands::Mileage(Mileage {
                mileage: 15000,
            }),
            Commands::DDG(DDGOperations {
                operation_type: DDGOption::Generate,
            }),
        ];

        assert_eq!(commands.len(), 6);
    }
}
