use anyhow::Result;
use colored::Colorize;
use lifestuff_types::mortgage::MortgageCommand;

use crate::dateinfo::get_date_from_string_arg;

pub fn create_mortgage(
    principal: f64,
    rate: f64,
    term_years: u32,
    monthly_payment: f64,
    start_date: &str,
    end_date: &str,
    notes: Option<String>,
) -> Result<i64> {
    let start_date = get_date_from_string_arg(Some(start_date), false)?
        .date()
        .to_string();

    let end_date = get_date_from_string_arg(Some(end_date), false)?
        .date()
        .to_string();

    let mortgage_id = conn.execute(
            "INSERT INTO mortgages (initial_principal, interest_rate, term_years, monthly_payment, start_date,end_date, notes)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![principal, rate / 100.0, term_years, monthly_payment, start_date, notes],
        )?;

    println!(
        "{}",
        format!("✓ Created mortgage with ID: {}", mortgage_id).green()
    );
    Ok(mortgage_id as i64)
}

pub fn handle_mortgage_operations(mortgage_args: MortgageCommand, verbose: bool) -> Result<()> {
    if verbose {
        println!("Mortgage Args: {:?}", mortgage_args);
    }

    match mortgage_args.action {
        lifestuff_types::mortgage::MortgageAction::Init(args) => {
            println!(
                "Initializing mortgage with principal: ${:.2}",
                args.principal
            );
            println!("Interest rate: {:.2}%", args.rate);
            println!("Term: {} years", args.term);
            println!("Monthly payment: ${:.2}", args.payment);
            println!("Start date: {}", args.start_date);
            println!("End date: {}", args.end_date);
            if let Some(notes) = args.notes {
                println!("Notes: {}", notes);
            }
            // TODO: Implement database storage
            Ok(())
        }
        lifestuff_types::mortgage::MortgageAction::Payment(args) => {
            println!("Recording payment for date: {}", args.date);
            if let Some(amount) = args.amount {
                println!("Payment amount: ${:.2}", amount);
            }
            if args.extra > 0.0 {
                println!("Extra principal payment: ${:.2}", args.extra);
            }
            if let Some(notes) = args.notes {
                println!("Notes: {}", notes);
            }
            // TODO: Implement payment recording
            Ok(())
        }
        lifestuff_types::mortgage::MortgageAction::Status => {
            println!("Current mortgage status:");
            // TODO: Implement status retrieval from database
            println!("Feature not yet implemented");
            Ok(())
        }
        lifestuff_types::mortgage::MortgageAction::History(args) => {
            if args.all {
                println!("Showing all payment history:");
            } else {
                println!("Showing last {} payments:", args.last);
            }
            // TODO: Implement payment history retrieval
            println!("Feature not yet implemented");
            Ok(())
        }
        lifestuff_types::mortgage::MortgageAction::Refinance(args) => {
            println!("Refinancing mortgage:");
            println!("New rate: {:.2}%", args.new_rate);
            println!("New term: {} years", args.new_term);
            println!("New payment: ${:.2}", args.new_payment);
            if let Some(date) = args.date {
                println!("Refinance date: {}", date);
            }
            if let Some(notes) = args.notes {
                println!("Notes: {}", notes);
            }
            // TODO: Implement refinancing logic
            Ok(())
        }
        lifestuff_types::mortgage::MortgageAction::Sync => {
            println!("Syncing mortgage data with cloud...");
            // TODO: Implement sync functionality
            println!("Feature not yet implemented");
            Ok(())
        }
    }
}
