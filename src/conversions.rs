use clap::{Args, Subcommand};
use strum::Display;

mod area;
pub use area::*;
mod distance;
pub use distance::*;

#[derive(Debug, Args)]
pub struct Conversions {
    #[command(subcommand)]
    convert_type: ConversionOption,
}

#[derive(Subcommand, Debug)]
enum ConversionOption {
    /// Area Conversions
    Area(area::AreaConversion),
    /// Distance Conversions
    Distance(distance::DistanceConversion),
}

#[repr(C)]
union UnitUnion {
    area: area::AreaUnits,
    distance: distance::DistanceUnits,
}

#[derive(Debug, Display)]
enum ConverstionType {
    Area,
    Distance,
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

fn format_conversion_output(
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
        let output = format_conversion_output(from, unit, val, &conversion, conversion_type);
        println!("{output}");
    }
}

pub fn perform_conversion(conversion_args: &Conversions, _verbose: bool) {
    match &conversion_args.convert_type {
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
