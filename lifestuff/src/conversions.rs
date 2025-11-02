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
    use lifestuff_types::conversions::area::AreaConversion;
    use lifestuff_types::conversions::area::AreaUnits;
    use lifestuff_types::conversions::distance::DistanceConversion;
    use lifestuff_types::conversions::distance::DistanceUnits;
    use lifestuff_types::conversions::{ConversionOption, Conversions};

    #[test]
    fn test_unit_conversions_area() {
        let from = UnitUnion {
            area: AreaUnits::SquareMetres,
        };
        let to = UnitUnion {
            area: AreaUnits::SqKilometres,
        };
        let result = unit_conversions(&from, &to, &1000.0, &ConversionType::Area);
        assert!((result - 0.001).abs() < 0.0001);
    }

    #[test]
    fn test_unit_conversions_distance() {
        let from = UnitUnion {
            distance: DistanceUnits::Metres,
        };
        let to = UnitUnion {
            distance: DistanceUnits::Kilometres,
        };
        let result = unit_conversions(&from, &to, &1000.0, &ConversionType::Distance);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_format_conversion_output_area() {
        let from = UnitUnion {
            area: AreaUnits::SquareMetres,
        };
        let to = UnitUnion {
            area: AreaUnits::SqKilometres,
        };
        let result = format_conversion_output(&from, &to, &1000.0, &0.001, &ConversionType::Area);
        assert_eq!(result, "1000 SquareMetres = 0.001 SqKilometres");
    }

    #[test]
    fn test_format_conversion_output_distance() {
        let from = UnitUnion {
            distance: DistanceUnits::Metres,
        };
        let to = UnitUnion {
            distance: DistanceUnits::Kilometres,
        };
        let result = format_conversion_output(&from, &to, &1000.0, &1.0, &ConversionType::Distance);
        assert_eq!(result, "1000 Metres = 1 Kilometres");
    }

    #[test]
    fn test_perform_conversion_area() {
        // Test that 1000 square metres = 0.001 square kilometres
        let from = UnitUnion {
            area: AreaUnits::SquareMetres,
        };
        let to = UnitUnion {
            area: AreaUnits::SqKilometres,
        };
        let result = unit_conversions(&from, &to, &1000.0, &ConversionType::Area);
        assert!((result - 0.001).abs() < f64::EPSILON);

        // Test the full conversion function
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
        // Test that 1000 metres = 1 kilometre
        let from = UnitUnion {
            distance: DistanceUnits::Metres,
        };
        let to = UnitUnion {
            distance: DistanceUnits::Kilometres,
        };
        let result = unit_conversions(&from, &to, &1000.0, &ConversionType::Distance);
        assert_eq!(result, 1.0);

        // Test the full conversion function
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
    fn test_conversion_type_routing() {
        // Test that different conversion types route to correct handlers
        let area_from = UnitUnion {
            area: AreaUnits::Acres,
        };
        let area_to = UnitUnion {
            area: AreaUnits::SquareMetres,
        };
        let area_result = unit_conversions(&area_from, &area_to, &1.0, &ConversionType::Area);
        assert!((area_result - 4046.856422).abs() < 0.001);

        let distance_from = UnitUnion {
            distance: DistanceUnits::Miles,
        };
        let distance_to = UnitUnion {
            distance: DistanceUnits::Kilometres,
        };
        let distance_result = unit_conversions(
            &distance_from,
            &distance_to,
            &1.0,
            &ConversionType::Distance,
        );
        assert!((distance_result - 1.609344).abs() < 0.001);

        // Verify the display implementation is used correctly in format_conversion_output
        let area_output = format_conversion_output(
            &area_from,
            &area_to,
            &1.0,
            &area_result,
            &ConversionType::Area,
        );
        assert!(area_output.contains("Acres"));
        assert!(area_output.contains("SquareMetres"));

        let distance_output = format_conversion_output(
            &distance_from,
            &distance_to,
            &1.0,
            &distance_result,
            &ConversionType::Distance,
        );
        assert!(distance_output.contains("Miles"));
        assert!(distance_output.contains("Kilometres"));
    }
}
