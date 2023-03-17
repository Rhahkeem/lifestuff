use clap::{Args, Parser, Subcommand};
use std::fmt;
use strum::Display;
mod area;
mod dateinfo;
mod distance;

#[derive(Parser, Debug)]
#[clap(
    author = "Me!",
    about = "Lifestyle library!",
    version = "0.1.0",
    long_about = "Something Something Something Daaaarksiiiide"
)]
struct Cli {
    #[arg(short, long, global=true, action=clap::ArgAction::SetTrue)]
    verbose: bool,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Display, Debug)]
enum Commands {
    /// Unit conversions
    Conversion(Conversions),
    /// Date Operations
    Dates(DateOperations),
}

#[derive(Args, Debug)]
struct DateOperations {
    #[command(subcommand)]
    operation_type: DateOption,
}

#[derive(Debug, Args)]
struct Conversions {
    #[command(subcommand)]
    convert_type: ConversionOption,
}

#[derive(Debug, Display)]
enum ConverstionType {
    Area,
    Distance,
}

#[derive(Subcommand, Debug)]
enum DateOption {
    ///Add two dates or time periods together
    Add(AreaConversion),
    ///Diff Two Dates
    Diff(dateinfo::Diff),
}

#[derive(Subcommand, Debug)]
enum ConversionOption {
    /// Area Conversions
    Area(AreaConversion),
    /// Distance Conversions
    Distance(DistanceConversion),
}

#[derive(Debug, Args)]
struct AreaConversion {
    #[arg(long, required = true)]
    from: area::AreaUnits,
    value: f64,
    #[arg(long, required = true)]
    to: Vec<area::AreaUnits>,
}

#[derive(Debug, Args)]
struct DistanceConversion {
    #[clap(long, required = true, display_order = 1)]
    from: distance::DistanceUnits,
    #[clap(display_order = 2)]
    value: f64,
    #[clap(long, required = true, display_order = 3)]
    to: Vec<distance::DistanceUnits>,
}

#[repr(C)]
union UnitUnion {
    area: area::AreaUnits,
    distance: distance::DistanceUnits,
}

impl fmt::Debug for UnitUnion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            write!(
                f,
                "UnitUnion elements Are: {}, and {}",
                &self.area, &self.distance
            )
        }
    }
}

fn unit_conversions(
    from: &UnitUnion,
    to: &UnitUnion,
    val: &f64,
    conversion_type: &ConverstionType,
) -> f64 {
    unsafe {
        match conversion_type {
            ConverstionType::Area => area::area_conversions(&from.area, &to.area, val),
            ConverstionType::Distance => {
                distance::distance_conversions(&from.distance, &to.distance, val)
            }
        }
    }
}

fn format_output(
    from_unit: &UnitUnion,
    to_unit: &UnitUnion,
    from_val: &f64,
    to_val: &f64,
    conversion_type: &ConverstionType,
) -> String {
    unsafe {
        match conversion_type {
            ConverstionType::Distance => format!(
                "{from_val} {:?} = {to_val} {:?}",
                from_unit.distance, to_unit.distance
            ),
            ConverstionType::Area => {
                format!(
                    "{from_val} {:?} = {to_val} {:?}",
                    from_unit.area, to_unit.area
                )
            }
        }
    }
}

fn conversion_prep(
    from: &UnitUnion,
    to: &Vec<UnitUnion>,
    val: &f64,
    conversion_type: &ConverstionType,
) {
    for unit in to {
        let conversion = unit_conversions(from, unit, val, conversion_type);
        let output = format_output(from, unit, val, &conversion, conversion_type);
        println!("{output}");
    }
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Conversion(args) => {
            if cli.verbose {
                println!("CLI Args: {:?}", cli);
            }
            match &args.convert_type {
                ConversionOption::Area(conversion_option) => {
                    conversion_prep(
                        &UnitUnion {
                            area: conversion_option.from,
                        },
                        &conversion_option
                            .to
                            .iter()
                            .map(|unit| UnitUnion { area: *unit })
                            .collect::<Vec<UnitUnion>>(),
                        &conversion_option.value,
                        &ConverstionType::Area,
                    );
                }
                ConversionOption::Distance(conversion_option) => {
                    conversion_prep(
                        &UnitUnion {
                            distance: conversion_option.from,
                        },
                        &conversion_option
                            .to
                            .iter()
                            .map(|unit| UnitUnion { distance: *unit })
                            .collect::<Vec<UnitUnion>>(),
                        &conversion_option.value,
                        &ConverstionType::Distance,
                    );
                }
            }
        }
        Commands::Dates(args) => match &args.operation_type {
            DateOption::Diff(diff_args) => {
                println!("{:?}", diff_args)
            }
            DateOption::Add(add_args) => {
                println!("{:?}", add_args)
            }
        },
        _ => todo!("Not done "),
    }
}

// compound interest
