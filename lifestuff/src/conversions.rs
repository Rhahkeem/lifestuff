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

#[cfg(test)]
mod tests {
    use super::*;
    use lifestuff_types::conversions::area::AreaUnits;
    use lifestuff_types::conversions::distance::DistanceUnits;
    use lifestuff_types::conversions::{ConversionOption, Conversions};
    use lifestuff_types::conversions::area::AreaConversion;
    use lifestuff_types::conversions::distance::DistanceConversion;

    #[test]
    fn test_unit_conversions_area() {
        let from = UnitUnion { area: AreaUnits::SquareMetres };
        let to = UnitUnion { area: AreaUnits::SqKilometres };
        let result = unit_conversions(&from, &to, &1000.0, &ConversionType::Area);
        assert!((result - 0.001).abs() < 0.0001);
    }

    #[test]
    fn test_unit_conversions_distance() {
        let from = UnitUnion { distance: DistanceUnits::Metres };
        let to = UnitUnion { distance: DistanceUnits::Kilometres };
        let result = unit_conversions(&from, &to, &1000.0, &ConversionType::Distance);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_format_conversion_output_area() {
        let from = UnitUnion { area: AreaUnits::SquareMetres };
        let to = UnitUnion { area: AreaUnits::SqKilometres };
        let result = format_conversion_output(&from, &to, &1000.0, &0.001, &ConversionType::Area);
        assert_eq!(result, "1000 SquareMetres = 0.001 SqKilometres");
    }

    #[test]
    fn test_format_conversion_output_distance() {
        let from = UnitUnion { distance: DistanceUnits::Metres };
        let to = UnitUnion { distance: DistanceUnits::Kilometres };
        let result = format_conversion_output(&from, &to, &1000.0, &1.0, &ConversionType::Distance);
        assert_eq!(result, "1000 Metres = 1 Kilometres");
    }

    #[test]
    fn test_perform_conversion_area() {
        let conversion = Conversions {
            convert_type: ConversionOption::Area(AreaConversion {
                from: AreaUnits::SquareMetres,
                to: vec![AreaUnits::SqKilometres],
                value: 1000.0,
            }),
        };
        let result = perform_conversion(conversion);
        assert!(result.is_ok());
    }

    #[test]
    fn test_perform_conversion_distance() {
        let conversion = Conversions {
            convert_type: ConversionOption::Distance(DistanceConversion {
                from: DistanceUnits::Metres,
                to: vec![DistanceUnits::Kilometres],
                value: 1000.0,
            }),
        };
        let result = perform_conversion(conversion);
        assert!(result.is_ok());
    }

    #[test]
    fn test_conversion_type_display() {
        assert_eq!(format!("{}", ConversionType::Area), "Area");
        assert_eq!(format!("{}", ConversionType::Distance), "Distance");
    }
}
