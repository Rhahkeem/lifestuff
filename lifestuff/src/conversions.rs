use anyhow::Result;
use lifestuff_types::conversions::area::AreaUnits;
use lifestuff_types::conversions::distance::DistanceUnits;
use lifestuff_types::conversions::{ConversionOption, Conversions};
use strum::Display;

mod area;
mod area_tests;
mod distance;

#[repr(C)]
union UnitUnion {
    area: AreaUnits,
    distance: DistanceUnits,
}

#[derive(Debug, Display)]
enum ConversionType {
    Area,
    Distance,
}

fn unit_conversions(
    from: &UnitUnion,
    to: &UnitUnion,
    val: &f64,
    conversion_type: &ConversionType,
) -> f64 {
    unsafe {
        match conversion_type {
            ConversionType::Area => area::area_conversions(&from.area, &to.area, val),
            ConversionType::Distance => {
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
    conversion_type: &ConversionType,
) -> String {
    unsafe {
        match conversion_type {
            ConversionType::Distance => format!(
                "{from_val} {:?} = {to_val} {:?}",
                from_unit.distance, to_unit.distance
            ),
            ConversionType::Area => {
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
    conversion_type: &ConversionType,
) {
    for unit in to {
        let conversion = unit_conversions(from, unit, val, conversion_type);
        let output = format_conversion_output(from, unit, val, &conversion, conversion_type);
        println!("{output}");
    }
}

pub fn perform_conversion(conversion_args: Conversions) -> Result<()> {
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
                &ConversionType::Area,
            );
            Ok(())
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
                &ConversionType::Distance,
            );
            Ok(())
        }
    }
}
