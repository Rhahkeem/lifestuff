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
